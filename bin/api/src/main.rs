use std::{env, str::FromStr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing, Json, Router,
};
use base64::{engine::general_purpose, Engine as _};
use gen::{
    enka_api::{api::Api, character::CharacterId, icon::IconData, DynamicImage},
    gen::{convert, generate as gen, get_default, ImageFormat, Lang, ScoreCounter},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{AllowOrigin, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub api: Arc<Api>,
    pub icons: Arc<IconData>,
}

impl AppState {
    pub fn new() -> Self {
        let api = Api::new();
        Self {
            icons: Arc::new(api.icon_data()),
            api: Arc::new(api),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/profile", routing::get(profile))
        .route("/generate", routing::get(generate))
        .with_state(AppState::new());
    let app = Router::new()
        .nest("/v1", app.clone())
        .nest("/", app)
        .layer(CorsLayer::new().allow_origin(AllowOrigin::any()));
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
pub struct ProfileQuery {
    pub uid: i32,
    pub lang: Option<String>,
    pub image_format: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    pub world_level: u8,
    pub level: u8,
    pub tower_floor_index: u8,
    pub tower_level_index: u8,
    pub uid: i32,
    pub achievement: u32,
    pub name: String,
    pub description: String,
    pub name_card: Option<String>,
    pub from_cache: bool,
    pub characters: Vec<UserCharacter>,
}

#[derive(Serialize)]
pub struct UserCharacter {
    pub ascension: u8,
    pub level: u8,
    pub element: String,
    pub xp: u32,
    pub name: String,
    pub icon: String,
    pub id: u32,
}

fn trim_image(img: Option<DynamicImage>, format: &str) -> Option<String> {
    if img.is_none() {
        return None;
    }
    let img = match ImageFormat::from_str(format) {
        Ok(f) => convert(img.unwrap(), f),
        Err(_) => None,
    };
    match img {
        Some(img) => Some(general_purpose::STANDARD_NO_PAD.encode(img.as_slice())),
        None => None,
    }
}

async fn profile(Query(q): Query<ProfileQuery>, State(s): State<AppState>) -> impl IntoResponse {
    let api = s.api;
    let lang = q.lang.unwrap_or("en".to_string());
    let format = q.image_format.unwrap_or("png".to_string());
    if !api.store.locale_list().contains(&&lang) {
        return (StatusCode::BAD_REQUEST, "Invalid language").into_response();
    }
    match api.simple(q.uid).await {
        Ok((usr, from_cache)) => {
            let profile = usr.profile();
            let mut characters = Vec::with_capacity(usr.characters.len());
            for (_, c) in &usr.characters {
                let icon = trim_image(c.image_icon(&api).await, &format);
                characters.push(UserCharacter {
                    ascension: c.ascension_level(),
                    level: c.level,
                    element: c.element.fight_prop_name().to_string(),
                    xp: c.xp,
                    name: match c.name(&api, lang.clone()) {
                        Ok(n) => n.to_string(),
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Could not get character name correctly",
                            )
                                .into_response()
                        }
                    },
                    icon: match icon {
                        Some(i) => i,
                        None => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Could not get character icon correctly",
                            )
                                .into_response()
                        }
                    },
                    id: c.id.0,
                });
            }
            let usr = User {
                world_level: profile.world_level(),
                level: profile.level(),
                tower_floor_index: profile.tower_floor_index(),
                tower_level_index: profile.tower_level_index(),
                uid: usr.uid,
                achievement: profile.achievement(),
                name: profile.nickname().clone(),
                description: profile.signature().clone(),
                name_card: trim_image(profile.name_card_image(&api).await, &format),
                from_cache,
                characters,
            };
            (StatusCode::OK, Json(usr)).into_response()
        }
        Err(_) => (
            StatusCode::FAILED_DEPENDENCY,
            "Enkanetwork has invalid response",
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct GenerateQuery {
    pub uid: i32,
    pub lang: Option<String>,
    pub image_format: Option<String>,
    pub cid: u32,
    pub counter: Option<String>,
}

async fn generate(Query(q): Query<GenerateQuery>, State(s): State<AppState>) -> impl IntoResponse {
    let api = s.api;
    let lang = q.lang.unwrap_or("en".to_string());
    let format = match ImageFormat::from_str(&q.image_format.unwrap_or("png".to_string())) {
        Ok(f) => f,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid image format").into_response(),
    };
    let counter = if let Some(c) = q.counter {
        ScoreCounter::from_str(&c).unwrap_or_else(|_| get_default(&q.cid))
    } else {
        get_default(&q.cid)
    };
    if !api.store.locale_list().contains(&&lang) {
        return (StatusCode::BAD_REQUEST, "Invalid language").into_response();
    }
    let (usr, from_cache) = match api.simple(q.uid).await {
        Ok((usr, from_cache)) => (usr, from_cache),
        Err(_) => {
            return (
                StatusCode::FAILED_DEPENDENCY,
                "Enkanetwork had invalid response",
            )
                .into_response()
        }
    };
    let character = match usr.character(CharacterId(q.cid)) {
        Some(c) => c,
        None => return (StatusCode::BAD_REQUEST, "Invalid character id").into_response(),
    };
    match gen(
        character.clone(),
        &api,
        &Lang::from(lang.as_str()),
        &s.icons,
        counter,
        format.clone(),
    )
    .await
    {
        Some(img) => {
            let mut headers = HeaderMap::new();
            headers.insert("X-From-Cache", from_cache.to_string().parse().unwrap());
            let mime = match format {
                ImageFormat::Png => "image/png",
                ImageFormat::Jpeg => "image/jpeg",
                _ => "image/raw",
            };
            headers.insert("Content-Type", mime.parse().unwrap());
            (StatusCode::OK, headers, img).into_response()
        }
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not generate image",
        )
            .into_response(),
    }
}

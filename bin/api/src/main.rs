use std::{env, str::FromStr, sync::Arc, time::UNIX_EPOCH};

use apitype::{GenerateQuery, ProfileQuery, User, UserCharacter};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing, Json, Router,
};
use base64::{engine::general_purpose, Engine as _};
use env_logger::Builder;
use gen::{
    enka_api::{api::Api, character::CharacterId, icon::IconData, DynamicImage},
    gen::{convert, generate as gen, get_default, ImageFormat, Lang, ScoreCounter},
};
use log::LevelFilter;
use tokio::time::Instant;
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
    dotenv::dotenv().ok();
    let level = env::var("LOG_LEVEL")
        .unwrap_or("INFO".to_string())
        .parse()
        .unwrap();
    Builder::new()
        .filter_level(LevelFilter::Warn)
        .filter_module("api", level)
        .filter_module("enka_api", level)
        .init();
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
    log::info!("Profile request {:?}", q);
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
                    element_name: c.element.fight_prop_name().to_string(),
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
                lastupdate: usr.lastupdate.duration_since(UNIX_EPOCH).unwrap().as_secs(),
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
async fn generate(Query(q): Query<GenerateQuery>, State(s): State<AppState>) -> impl IntoResponse {
    log::info!("Generate request {:?}", q);
    let now = Instant::now();
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
            headers.insert("X-Score-Counter", counter.to_string().parse().unwrap());
            let mime = match format {
                ImageFormat::Png => "image/png",
                ImageFormat::Jpeg => "image/jpeg",
                _ => "image/raw",
            };
            headers.insert("Content-Type", mime.parse().unwrap());
            log::info!("Generated image in {}ms", now.elapsed().as_millis());
            (StatusCode::OK, headers, img).into_response()
        }
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not generate image",
        )
            .into_response(),
    }
}

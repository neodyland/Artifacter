use std::{env, str::FromStr, sync::Arc, time::UNIX_EPOCH};

use apitype::genshin::{GenerateQuery, ProfileQuery, User, UserCharacter};
use apitype::hsr::{
    GenerateQuery as HsrGenerateQuery, ProfileQuery as HsrProfileQuery, User as HsrUser,
    UserCharacter as HsrUserCharacter,
};
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
use hsr_gen::base::random_base_image;
use hsr_gen::{
    base::{get_base_image, BaseImage},
    format::ImageFormat as HsrImageFormat,
    gen::{generate as hsr_gen, ScoreCounter as HsrScoreCounter},
    mihoyo_api::api::Api as MihoyoApi,
};
use log::LevelFilter;
use tokio::time::Instant;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub api: Arc<Api>,
    pub icons: Arc<IconData>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
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

#[derive(Clone)]
pub struct HsrAppState {
    pub api: Arc<MihoyoApi>,
}

impl Default for HsrAppState {
    fn default() -> Self {
        Self::new()
    }
}

impl HsrAppState {
    pub fn new() -> Self {
        let api = MihoyoApi::new();
        Self { api: Arc::new(api) }
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
        .filter_module("mihoyo_api", level)
        .init();
    let hsr = Router::new()
        .route("/profile", routing::get(hsr_profile))
        .route("/generate", routing::get(hsr_generate))
        .with_state(HsrAppState::new());
    let app = Router::new()
        .route("/profile", routing::get(profile))
        .route("/generate", routing::get(generate))
        .nest("/hsr", hsr)
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
    img.as_ref()?;
    let img = match ImageFormat::from_str(format) {
        Ok(f) => convert(img.unwrap(), f),
        Err(_) => None,
    };
    img.map(|img| general_purpose::STANDARD_NO_PAD.encode(img.as_slice()))
}

async fn profile(Query(q): Query<ProfileQuery>, State(s): State<AppState>) -> impl IntoResponse {
    log::info!("Profile request {:?}", q);
    let api = s.api;
    let lang = Lang::from(q.lang.unwrap_or("en".to_string()).as_str()).to_string();
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
    let lang = Lang::from(q.lang.unwrap_or("en".to_string()).as_str()).to_string();
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

async fn hsr_profile(
    Query(q): Query<HsrProfileQuery>,
    State(s): State<HsrAppState>,
) -> impl IntoResponse {
    log::info!("HSR Profile request {:?}", q);
    let api = s.api;
    let lang = q.lang.unwrap_or("en".to_string());
    let format = q.image_format.unwrap_or("png".to_string());
    match api.simple(q.uid, lang.to_string()).await {
        Ok((usr, from_cache)) => {
            let mut characters = Vec::with_capacity(usr.characters.len());
            for c in &usr.characters {
                let icon = trim_image(api.asset(&c.icon).await.ok(), &format);
                characters.push(HsrUserCharacter {
                    level: c.level,
                    element_name: c.element.name.clone(),
                    element: c.element.id.clone(),
                    name: c.name.clone(),
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
                    id: match c.id.parse() {
                        Ok(i) => i,
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Could not get character id correctly",
                            )
                                .into_response()
                        }
                    },
                    path: c.path.id.clone(),
                    path_name: c.path.name.clone(),
                });
            }
            let usr = HsrUser {
                world_level: usr.world_level,
                level: usr.level,
                uid: usr.uid,
                achievement: usr.advancements as u32,
                name: usr.name.clone(),
                description: usr.description.clone(),
                avatar: trim_image(api.asset(&usr.avatar_icon).await.ok(), &format),
                from_cache,
                characters,
                lastupdate: usr.lastupdate.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            };
            (StatusCode::OK, Json(usr)).into_response()
        }
        Err(_) => (
            StatusCode::FAILED_DEPENDENCY,
            "HoyoAPI has invalid response",
        )
            .into_response(),
    }
}

async fn hsr_generate(
    Query(q): Query<HsrGenerateQuery>,
    State(s): State<HsrAppState>,
) -> impl IntoResponse {
    log::info!("HSR Generate request {:?}", q);
    let now = Instant::now();
    let api = s.api;
    let lang = q.lang.unwrap_or("en".to_string());
    let format = match HsrImageFormat::from_str(&q.image_format.unwrap_or("png".to_string())) {
        Ok(f) => f,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid image format").into_response(),
    };
    let counter = if let Some(c) = q.counter {
        HsrScoreCounter::from_str(&c).unwrap_or(HsrScoreCounter::Attack)
    } else {
        HsrScoreCounter::Attack
    };
    let base_img = if let Some(b) = q.base_img {
        BaseImage::from_str(&b).unwrap_or(BaseImage::Belobog)
    } else {
        random_base_image()
    };
    let (usr, from_cache) = match api.simple(q.uid, lang.clone()).await {
        Ok((usr, from_cache)) => (usr, from_cache),
        Err(_) => {
            return (
                StatusCode::FAILED_DEPENDENCY,
                "Hoyo API had invalid response",
            )
                .into_response()
        }
    };
    let character = match usr.characters.iter().find(|c| c.id == q.cid.to_string()) {
        Some(c) => c,
        None => return (StatusCode::BAD_REQUEST, "Invalid character id").into_response(),
    };
    match hsr_gen(
        &api,
        character,
        get_base_image(base_img),
        format.clone(),
        &lang,
        &counter,
    )
    .await
    {
        Some(img) => {
            let mut headers = HeaderMap::new();
            headers.insert("X-From-Cache", from_cache.to_string().parse().unwrap());
            headers.insert("X-Score-Counter", counter.to_string().parse().unwrap());
            let mime = match format {
                HsrImageFormat::Png => "image/png",
                HsrImageFormat::Jpeg => "image/jpeg",
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

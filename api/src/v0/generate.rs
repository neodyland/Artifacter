use axum::{
    body::Body,
    extract::{Query, State},
    response::Response,
};
use enkanetwork_rs::CharacterId;
use gen::{generate, ImageFormat, Lang, ScoreCounter};
use serde::Deserialize;

use crate::GlobalState;

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(default)]
    pub lang: Lang,
    #[serde(default)]
    pub format: ImageFormat,
    #[serde(default)]
    pub counter: ScoreCounter,
    pub uid: i32,
    pub cid: u32,
}

pub async fn handler(
    Query(params): Query<Params>,
    State(state): State<GlobalState>,
) -> Response<Body> {
    let api = state.api.lock().await;
    let icons = state.icons.lock().await;
    let user = api.simple(params.uid).await;
    if user.is_err() {
        return Response::builder()
            .status(404)
            .body(Body::from("User Not Found"))
            .unwrap();
    }
    let (user, _) = user.unwrap();
    let character = user.character(CharacterId(params.cid));
    if character.is_none() {
        return Response::builder()
            .status(404)
            .body(Body::from("Character Not Found"))
            .unwrap();
    };
    let image = generate(
        character.unwrap().to_owned(),
        &api,
        &params.lang,
        &icons,
        params.counter,
        params.format,
    )
    .await;
    if image.is_none() {
        return Response::builder()
            .status(500)
            .body(Body::from("Internal Server Error"))
            .unwrap();
    }
    Response::builder()
        .status(200)
        .body(Body::from(image.unwrap()))
        .unwrap()
}

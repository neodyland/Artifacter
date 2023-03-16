mod generate;

use axum::{routing, Router};

use crate::GlobalState;

pub fn get_router() -> Router<GlobalState> {
    Router::new().route("/generate", routing::get(generate::handler))
}

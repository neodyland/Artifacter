use std::sync::{atomic::AtomicBool, Arc};

use tokio::sync::Mutex;

use crate::{
    api::Api,
    cache::{Cache, HsrCache},
    db::{connect, PgPool},
};

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
    pub started: Arc<AtomicBool>,
    pub api: Api,
    pub cache: Arc<Mutex<Cache>>,
    pub hsr_cache: Arc<Mutex<HsrCache>>,
}

impl State {
    pub async fn new() -> Self {
        let db = connect().await;
        Self {
            db,
            started: Arc::new(AtomicBool::new(false)),
            api: Api::new(),
            cache: Arc::new(Mutex::new(Cache::new())),
            hsr_cache: Arc::new(Mutex::new(HsrCache::new())),
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, State, Error>;

use std::sync::{atomic::AtomicBool, Arc};

use tokio::sync::Mutex;

use crate::db::{connect, PgPool};

pub struct State {
    pub db: Arc<Mutex<PgPool>>,
    pub started: AtomicBool,
}

impl State {
    pub async fn new() -> Self {
        let db = connect().await;
        Self {
            db: Arc::new(Mutex::new(db)),
            started: AtomicBool::new(false),
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Arc<Mutex<State>>, Error>;

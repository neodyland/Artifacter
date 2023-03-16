use std::{env, net::SocketAddr, sync::Arc};

use axum::{Router, Server};
use enkanetwork_rs::{EnkaNetwork, IconData};
use tokio::sync::Mutex;

mod v0;

fn main() -> anyhow::Result<()> {
    let api = EnkaNetwork::new()?;
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(_main(api))
}

#[derive(Clone)]
pub struct GlobalState {
    pub icons: Arc<Mutex<IconData>>,
    pub api: Arc<Mutex<EnkaNetwork>>,
}

impl GlobalState {
    pub async fn new(api: EnkaNetwork) -> Self {
        Self {
            icons: Arc::new(Mutex::new(IconData::load(&api).await)),
            api: Arc::new(Mutex::new(api)),
        }
    }
}

async fn _main(api: EnkaNetwork) -> anyhow::Result<()> {
    let state = GlobalState::new(api).await;
    let port = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .unwrap();
    let server = Server::bind(&SocketAddr::try_from(([0, 0, 0, 0], port)).unwrap());
    let v0_router = v0::get_router().with_state(state);
    let app = Router::new().nest("/v0", v0_router);
    server.serve(app.into_make_service()).await?;
    Ok(())
}

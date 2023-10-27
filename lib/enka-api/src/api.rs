use std::time::SystemTime;

use crate::{
    cache::Cache,
    store::Store,
    user::{ApiRawUser, ApiUser},
};
use image::{io::Reader as ImageReader, DynamicImage};
use reqwest::{Client, Error as ReqwestError};

pub struct Api {
    pub client: Client,
    pub store: Store,
    pub cache: Cache,
}

const USER_AGENT: &str = "EnkaApi/0.1.0 (+https://artifacter.neody.land/)";

impl Api {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .expect("Failed to build reqwest client"),
            store: Store::new(),
            cache: Cache::new(),
        }
    }
    async fn request(&self, uri: &str) -> Result<Vec<u8>, ReqwestError> {
        log::info!("requesting {}", uri);
        let request = self.client.get(uri);
        let body = request.send().await?;
        let body = body.error_for_status()?;
        let body = body.bytes().await?;
        Ok(body.to_vec())
    }
    async fn assets(&self, path: impl AsRef<str>) -> Result<Vec<u8>, String> {
        if let Ok((buf, _time)) = self.cache.get(&path).await {
            return Ok(buf.to_vec());
        }
        let url = format!("https://enka.network/{}", path.as_ref());
        let body = match self.request(&url).await {
            Ok(body) => body,
            Err(e) => return Err(format!("{}", e)),
        };
        self.cache
            .set(path.as_ref().to_string(), &body, SystemTime::now())
            .await
            .ok();
        Ok(body)
    }
    pub async fn ui_image(&self, path: impl AsRef<str>) -> Result<DynamicImage, String> {
        let url = format!("ui/{}.png", path.as_ref());
        let body = self.assets(&url).await?;
        let reader = ImageReader::new(std::io::Cursor::new(body));
        let reader = match reader.with_guessed_format() {
            Ok(img) => img,
            Err(e) => return Err(format!("{}", e)),
        };
        match reader.decode() {
            Ok(img) => Ok(img),
            Err(e) => Err(format!("{}", e)),
        }
    }
    pub fn get_store(&self) -> &Store {
        &self.store
    }
    async fn fetch_user(&self, uid: i32) -> Result<ApiRawUser, Option<ReqwestError>> {
        let contents = self
            .request(&format!("https://enka.network/api/uid/{}/", uid))
            .await?;
        let lastupdate = SystemTime::now();
        Ok(ApiRawUser::from_raw(contents, uid, lastupdate))
    }
    async fn set_cache(&self, data: &ApiRawUser) -> std::io::Result<()> {
        let now = SystemTime::now();
        self.cache
            .set(format!("user/{}", data.uid()), data.contents(), now)
            .await
    }
    async fn find_cache(&self, uid: i32) -> Option<ApiRawUser> {
        let (buf, modtime) = self.cache.get(format!("user/{}", uid)).await.ok()?;
        Some(ApiRawUser::from_raw(buf.to_vec(), uid, modtime))
    }
    pub async fn simple(&self, uid: i32) -> Result<(ApiUser, bool), String> {
        match self.find_cache(uid).await {
            Some(cache) => {
                let data = cache.resolve(self)?;
                match self.reload(&data).await {
                    Ok(u) => match u {
                        Some(new) => Ok((new, false)),
                        None => Ok((data, false)),
                    },
                    Err(e) => {
                        if e.contains("424") {
                            Ok((data, true))
                        } else {
                            Err(e)
                        }
                    }
                }
            }
            None => {
                let userdata = self.fetch_user(uid).await;
                match userdata {
                    Ok(userdata) => {
                        let _ = self.set_cache(&userdata).await;
                        Ok((userdata.resolve(self)?, false))
                    }
                    Err(e) => Err(match e {
                        Some(e) => format!("{}", e),
                        None => String::from("unknown error"),
                    }),
                }
            }
        }
    }
    async fn reload(&self, data: &ApiUser) -> Result<Option<ApiUser>, String> {
        let lastupdate = SystemTime::now();
        if data.reload_time() >= lastupdate {
            Ok(None)
        } else {
            let raw = self.fetch_user(data.uid()).await;
            match raw {
                Ok(raw) => {
                    let _ = self.set_cache(&raw).await;
                    Ok(Some(raw.resolve(self)?))
                }
                Err(e) => Err(match e {
                    Some(e) => format!("{}", e),
                    None => String::from("unknown error"),
                }),
            }
        }
    }
}

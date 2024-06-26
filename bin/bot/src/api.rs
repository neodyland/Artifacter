use std::env;

use apitype::genshin::User;
use apitype::hsr::User as HsrUser;
use reqwest::{header::HeaderMap, Client};

#[derive(Clone)]
pub struct Api {
    pub client: Client,
    pub hostname: String,
}

impl Api {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            hostname: env::var("API").unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }
    async fn request(
        &self,
        path: String,
        params: Vec<(String, String)>,
    ) -> Result<(Vec<u8>, HeaderMap), Box<dyn std::error::Error + Send + Sync>> {
        let params = params
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        let url = format!("{}/v1/{}?{}", self.hostname, path, params);
        let res = self.client.get(url).send().await?;
        let headers = res.headers().clone();
        if res.status().is_client_error() || res.status().is_server_error() {
            let err = res.text().await?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err,
            )));
        }
        Ok((res.bytes().await?.to_vec(), headers))
    }
    pub async fn profile(
        &self,
        uid: String,
        lang: Option<String>,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let mut params = vec![("uid".to_string(), uid)];
        if let Some(lang) = lang {
            params.push(("lang".to_string(), lang));
        }
        let (res, _h) = self.request("profile".to_string(), params).await?;
        let user = serde_json::from_slice(&res)?;
        Ok(user)
    }
    pub async fn hsr_profile(
        &self,
        uid: String,
        lang: Option<String>,
    ) -> Result<HsrUser, Box<dyn std::error::Error + Send + Sync>> {
        let mut params = vec![("uid".to_string(), uid)];
        if let Some(lang) = lang {
            params.push(("lang".to_string(), lang));
        }
        let (res, _h) = self.request("hsr/profile".to_string(), params).await?;
        let user = serde_json::from_slice(&res)?;
        Ok(user)
    }
    pub async fn generate(
        &self,
        lang: Option<String>,
        uid: String,
        character: String,
        score: Option<String>,
        format: String,
    ) -> Result<(Vec<u8>, String), Box<dyn std::error::Error + Send + Sync>> {
        let mut params = vec![
            ("uid".to_string(), uid),
            ("cid".to_string(), character),
            ("image_format".to_string(), format),
            ("lang".to_string(), lang.unwrap_or_else(|| "en".to_string())),
        ];
        if let Some(score) = score {
            params.push(("counter".to_string(), score));
        }
        let (buf, h) = self.request("generate".to_string(), params).await?;
        Ok((
            buf,
            h.get("X-Score-Counter")
                .unwrap()
                .to_str()?
                .to_string()
                .to_lowercase(),
        ))
    }
    pub async fn hsr_generate(
        &self,
        lang: Option<String>,
        uid: String,
        character: String,
        score: Option<String>,
        format: String,
        base_img: Option<String>,
    ) -> Result<(Vec<u8>, String), Box<dyn std::error::Error + Send + Sync>> {
        let mut params = vec![
            ("uid".to_string(), uid),
            ("cid".to_string(), character),
            ("image_format".to_string(), format),
            ("lang".to_string(), lang.unwrap_or_else(|| "en".to_string())),
        ];
        if let Some(base_img) = base_img {
            params.push(("base_img".to_string(), base_img));
        }
        if let Some(score) = score {
            params.push(("counter".to_string(), score));
        }
        let (buf, h) = self.request("hsr/generate".to_string(), params).await?;
        Ok((
            buf,
            h.get("X-Score-Counter")
                .unwrap()
                .to_str()?
                .to_string()
                .to_lowercase(),
        ))
    }
}

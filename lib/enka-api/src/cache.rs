use filetime::FileTime;
use std::{path::Path, time::SystemTime};
use tokio::{
    fs::{create_dir_all, try_exists, write, File},
    io::AsyncReadExt,
};

const CACHE_DIR: &str = "./cache";

pub struct Cache {}

impl Cache {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) async fn set(
        &self,
        key: String,
        value: &[u8],
        time: SystemTime,
    ) -> std::io::Result<()> {
        let path = Path::new(CACHE_DIR).join(key);
        if let Some(p) = path.parent() {
            if let Ok(false) = try_exists(p).await {
                create_dir_all(p).await?;
            }
        }
        write(&path, value).await?;
        filetime::set_file_mtime(&path, FileTime::from_system_time(time))?;
        Ok(())
    }
    pub(crate) async fn get(
        &self,
        path: impl AsRef<str>,
    ) -> std::io::Result<(Vec<u8>, SystemTime)> {
        let mut f = File::open(path.as_ref()).await?;
        let meta = f.metadata().await?;
        let modtime = meta.modified()?;
        let mut buf = Vec::with_capacity(meta.len() as usize);
        f.read_to_end(&mut buf).await?;
        Ok((buf, modtime))
    }
}

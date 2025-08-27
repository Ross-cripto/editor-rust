use std::path::PathBuf;
use std::sync::Arc;

use crate::error::Error;

pub async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

pub async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(Error::from)?;

    Ok((path, contents))
}

pub async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        let handle = rfd::AsyncFileDialog::new().set_title("Choose a file to save").save_file().await.ok_or(Error::DialogClosed)?;
        handle.path().to_owned()
    };

    tokio::fs::write(&path, text).await.map_err(Error::from)?;
    Ok(path)
}

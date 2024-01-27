use std::env;
use tokio;
use anyhow::{Result, anyhow};

pub async fn get_session_cookie() -> Result<String> {
    let exe_path = env::current_exe()?;
    let file_path = exe_path.parent().ok_or_else(|| anyhow!("Failed to get parent of executable"))?
        .join("cookies.txt");


    let cookie = match tokio::fs::read_to_string(file_path).await {
        Ok(value) => value,
        Err(_) => String::from("Currently no session cookie set. Try setting a cookie first.")
    };

    return Ok(cookie);
}

pub async fn set_session_cookie(cookie: &String) -> Result<()> {
    let mut new_cookie = cookie.clone();

    if !new_cookie.starts_with("session=") {
        new_cookie = format!("session={}", cookie);
    }

    let exe_path = env::current_exe()?;
    let file_path = exe_path
        .parent()
        .ok_or_else(|| anyhow!("Failes to get parent of executable"))?
        .join("cookies.txt");

    if !file_path.exists() {
        tokio::fs::write(file_path.clone(), "").await?;
    }

    tokio::fs::write(file_path, new_cookie).await?;

    return Ok(());
}

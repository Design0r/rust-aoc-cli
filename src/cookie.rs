use anyhow::{anyhow, Result};
use std::{env, fs};

pub fn get_session_cookie() -> Result<String> {
    let exe_path = env::current_exe()?;
    let file_path = exe_path
        .parent()
        .ok_or_else(|| anyhow!("Failed to get parent of executable"))?
        .join("cookies.txt");

    let cookie = fs::read_to_string(file_path);

    return Ok(cookie?);
}

pub fn set_session_cookie(cookie: &String) -> Result<()> {
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
        fs::write(file_path.clone(), "")?;
    }

    fs::write(file_path, new_cookie)?;

    return Ok(());
}

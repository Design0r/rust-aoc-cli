use std::{env, io};
use tokio;

pub async fn get_session_cookie() -> String {
    let exe_path = env::current_exe().expect("Failed to get the current executable path");
    let file_path = exe_path
        .parent()
        .expect("Failed to get parent directory of executable path")
        .join("cookies.txt");

    let cookie;

    match tokio::fs::read_to_string(file_path).await {
        Ok(value) => cookie = value,
        Err(_) => {
            cookie = String::from("Currently no session cookie set. Try setting a cookie first.")
        }
    };

    return cookie;
}

pub async fn set_session_cookie(cookie: &String) -> io::Result<()> {
    let mut new_cookie = cookie.clone();

    if !new_cookie.starts_with("session=") {
        new_cookie = format!("session={}", cookie);
    }

    let exe_path = env::current_exe()?;
    let file_path = exe_path
        .parent()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Executable has no parent directory",
        ))?
        .join("cookies.txt");

    if !file_path.exists() {
        tokio::fs::write(file_path.clone(), "").await?;
    }

    tokio::fs::write(file_path, new_cookie).await?;

    return Ok(());
}

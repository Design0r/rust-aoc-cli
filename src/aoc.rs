use crate::args::{ DownloadArgs, SubmitArgs};
use reqwest::StatusCode;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use crate::cookie;
use crate::utils::{self, get_latest_aoc_year};
use anyhow::{Result, anyhow};

pub async fn request_input(url: &String ) -> Result<String>{
    let cookie = cookie::get_session_cookie().await?;
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie)?);

    let req= client.get(url).headers(headers).build()?;
    let res = client.execute(req).await?;
    if res.status() != StatusCode::OK{
        return Err(anyhow!("Have you set a valid session cookie?"));
    }

    return Ok(res.text().await?);
}


pub async fn send_solution(url: &String, args: &SubmitArgs) -> Result<()>{
    let cookie = cookie::get_session_cookie().await?;
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie)?);

    let data = [("level", args.part), ("answer", args.solution)];
    let req= client.post(url).headers(headers).form(&data).build()?;
    let res = client.execute(req).await?;

    if res.status() != StatusCode::OK{
        return Err(anyhow!("Have you set a valid session cookie?"));
    }

    let article = utils::get_article_content(res.text().await?)?;

    println!("{}", article);
    return Ok(());
}

pub fn get_download_url(args: &DownloadArgs) -> String{
    let year = match args.year {
        Some(y) => y.to_string(),
        None => {get_latest_aoc_year().to_string()}
    };

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, args.day );
    return url;
}

pub fn get_submit_url(args: &SubmitArgs) -> String{
    let url = format!("https://adventofcode.com/{}/day/{}/answer", args.year, args.day);
    return url;
}

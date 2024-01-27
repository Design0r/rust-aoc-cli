use crate::args::{ DownloadArgs, SubmitArgs};
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use crate::cookie;
use crate::utils;
use anyhow::Result;

pub async fn request_input(url: &String ) -> Result<String>{
    let cookie = cookie::get_session_cookie().await?;
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie)?);

    let req= client.get(url).headers(headers).build()?;
    let res = client.execute(req).await?;

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

    let article = utils::get_article_content(res.text().await?)?;

    println!("{}", article);
    return Ok(());
}

pub fn get_download_url(args: &DownloadArgs) -> String{
    let url = format!("https://adventofcode.com/{}/day/{}/input", args.year, args.day );
    return url;
}

pub fn get_submit_url(args: &SubmitArgs) -> String{
    let url = format!("https://adventofcode.com/{}/day/{}/answer", args.year, args.day);
    return url;
}

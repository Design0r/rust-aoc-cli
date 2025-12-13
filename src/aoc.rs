use crate::args::{DownloadArgs, SubmitArgs};
use crate::cookie;
use crate::utils::{self, get_latest_aoc_year};
use anyhow::{Result, anyhow};
use reqwest::StatusCode;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};

pub fn request_input(url: &String) -> Result<String> {
    let cookie =
        cookie::get_session_cookie().expect("failed to get session cookie, try setting one first.");
    let client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie)?);

    let req = client.get(url).headers(headers).build()?;
    let res = client.execute(req)?;
    if res.status() != StatusCode::OK {
        println!("unsuccessful download request, have you set a valid session cookie?");
        std::process::exit(1);
    }

    return Ok(res.text()?);
}

pub fn send_solution(url: &String, args: &SubmitArgs) -> Result<()> {
    let cookie = cookie::get_session_cookie()?;
    let client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie)?);

    let data = [
        ("level", args.part.to_string()),
        ("answer", args.solution.clone()),
    ];
    let req = client.post(url).headers(headers).form(&data).build()?;
    let res = client.execute(req)?;

    if res.status() != StatusCode::OK {
        return Err(anyhow!("have you set a valid session cookie?"));
    }

    let article = utils::get_article_content(res.text()?)?;

    println!("{}", article);
    return Ok(());
}

pub fn get_download_url(args: &DownloadArgs) -> String {
    let year = match args.year {
        Some(y) => y.to_string(),
        None => get_latest_aoc_year().to_string(),
    };

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, args.day);
    return url;
}

pub fn get_submit_url(args: &SubmitArgs) -> String {
    let url = format!(
        "https://adventofcode.com/{}/day/{}/answer",
        args.year, args.day
    );
    return url;
}

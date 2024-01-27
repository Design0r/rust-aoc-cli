use crate::args::{ DownloadArgs, SubmitArgs};
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use crate::cookie;
use crate::utils;

pub async fn request_input(url: &String ) -> String{
    let cookie = cookie::get_session_cookie().await;
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());

    let req= client.get(url).headers(headers).build().unwrap();
    let res = client.execute(req).await.unwrap();

    return res.text().await.unwrap();
}


pub async fn send_solution(url: &String, args: &SubmitArgs) {
    let cookie = cookie::get_session_cookie().await;
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());

    let data = [("level", args.part), ("answer", args.solution)];
    let req= client.post(url).headers(headers).form(&data).build().unwrap();
    let res = client.execute(req).await.unwrap();

    let article = utils::get_article_content(res.text().await.unwrap()); 

    println!("{}", article);
}

pub fn get_download_url(args: &DownloadArgs) -> Option<String>{
    let url = format!("https://adventofcode.com/{}/day/{}/input", args.year, args.day );
    return Some(url);
}

pub fn get_submit_url(args: &SubmitArgs) -> Option<String>{
    let url = format!("https://adventofcode.com/{}/day/{}/answer", args.year, args.day);
    return Some(url);
}

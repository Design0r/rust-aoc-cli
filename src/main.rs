mod args;
mod cookie;
mod utils;
mod aoc;

use tokio;
use args::{AocArgs, Command, DownloadArgs, SubmitArgs};
use clap::Parser;



async fn exec_download(args: &DownloadArgs) {
    let url = aoc::get_download_url(args).unwrap();
    let input = aoc::request_input(&url).await;
    utils::scaffold_project(args, &input).await;
}

async fn exec_submit(args: &SubmitArgs) {
    let url = aoc::get_submit_url(args).unwrap();
    aoc::send_solution(&url, args).await;
}

async fn exec_cookie(cookie: &Option<String>) {
    match cookie {
        Some(value) => cookie::set_session_cookie(&value).await.unwrap(),
        None => println!("{}", cookie::get_session_cookie().await),
    };
}


#[tokio::main]
async fn main() {
    let args = AocArgs::parse();
    args::check_args(&args);

    match args.command {
        Command::Download(args) => exec_download(&args).await,
        Command::Submit(args) => exec_submit(&args).await,
        Command::Cookie { cookie } => exec_cookie(&cookie).await,
    }
}

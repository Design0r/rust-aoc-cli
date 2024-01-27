mod args;
mod cookie;
mod utils;
mod aoc;

use tokio;
use args::{AocArgs, Command, DownloadArgs, SubmitArgs};
use clap::Parser;
use anyhow::Result;


async fn exec_download(args: &DownloadArgs) -> Result<()>{
    let url = aoc::get_download_url(args);
    let input = aoc::request_input(&url).await?;
    utils::scaffold_project(args, &input).await?;

    return Ok(());
}

async fn exec_submit(args: &SubmitArgs) -> Result<()>{
    let url = aoc::get_submit_url(args);
    aoc::send_solution(&url, args).await?;

    return Ok(());
}

async fn exec_cookie(cookie: &Option<String>) -> Result<()>{
    match cookie {
        Some(value) => cookie::set_session_cookie(&value).await.unwrap(),
        None => println!("{}", cookie::get_session_cookie().await?),
    };

    return Ok(());
}


#[tokio::main]
async fn main() -> Result<()>{
    let args = AocArgs::parse();
    args::check_args(&args);

    match args.command {
        Command::Download(args) => exec_download(&args).await?,
        Command::Submit(args) => exec_submit(&args).await?,
        Command::Cookie { cookie } => exec_cookie(&cookie).await?,
    }

    return Ok(());
}

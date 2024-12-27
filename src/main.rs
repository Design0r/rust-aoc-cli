mod aoc;
mod args;
mod cookie;
mod utils;

use anyhow::Result;
use args::{AocArgs, Command, DownloadArgs, SubmitArgs};
use clap::Parser;

fn exec_download(args: &DownloadArgs) -> Result<()> {
    let url = aoc::get_download_url(args);
    let input = aoc::request_input(&url)?;
    let _ = utils::scaffold_project(args, &input)?;

    return Ok(());
}

fn exec_submit(args: &SubmitArgs) -> Result<()> {
    let url = aoc::get_submit_url(args);
    aoc::send_solution(&url, args)?;

    return Ok(());
}

fn exec_cookie(cookie: &Option<String>) -> Result<()> {
    match cookie {
        Some(value) => cookie::set_session_cookie(&value).unwrap(),
        None => println!("{}", cookie::get_session_cookie()?),
    };

    return Ok(());
}

fn main() -> Result<()> {
    let args = AocArgs::parse();
    args::check_args(&args);

    let _ = match args.command {
        Command::Download(args) => exec_download(&args),
        Command::Submit(args) => exec_submit(&args),
        Command::Cookie { cookie } => exec_cookie(&cookie),
    };

    return Ok(());
}

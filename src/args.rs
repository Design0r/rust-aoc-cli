use crate::utils;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct AocArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Download(DownloadArgs),
    Submit(SubmitArgs),
    Cookie { cookie: Option<String> },
}

#[derive(Args, Debug)]
pub struct DownloadArgs {
    pub path: Option<String>,
    #[arg(short, long)]
    pub day: u32,
    #[arg(short, long)]
    pub year: u32,
}

#[derive(Args, Debug)]
pub struct SubmitArgs {
    pub solution: u32,
    #[arg(short, long)]
    pub day: u32,
    #[arg(short, long)]
    pub year: u32,
    #[arg(short, long)]
    pub part: u32,
}

#[derive(Args, Debug)]
pub struct CookieArgs {
    pub cookie: Option<String>,
}

pub fn check_args(args: &AocArgs) {
    let aoc_year = utils::get_latest_aoc_year() as u32;

    match &args.command {
        Command::Download(a) => {
            if a.year < 2015 || a.year > aoc_year {
                println!(
                    "The Year argument has to be a number between 2015 and {}",
                    aoc_year
                );
                std::process::exit(1);
            }

            if a.day > 25 {
                println!("The Day argument has to be a number between 1 and 25");
                std::process::exit(1);
            }
        }
        Command::Submit(a) => {
            if a.year < 2015 || a.year > aoc_year {
                println!(
                    "The Year argument has to be a number between 2015 and {}",
                    aoc_year
                );
                std::process::exit(1);
            }

            if a.day > 25 {
                println!("The Day argument has to be a number between 1 and 25");
                std::process::exit(1);
            }

            if a.part < 1 || a.part > 2 {
                println!("The Part argument has to be a number between 1 and 2");
                std::process::exit(1);
            }
        }
        Command::Cookie { cookie: _ } => {}
    }
}

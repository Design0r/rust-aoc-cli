use crate::utils::get_latest_aoc_year;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct AocArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Download an Advent of Code puzzle
    Download(DownloadArgs),
    /// Submit your Advent of Code solution
    Submit(SubmitArgs),
    /// Set your Advent of Code session cookie
    Cookie { cookie: Option<String> },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Language {
    Rust = 0,
    Rs = 1,
    Python = 2,
    Py = 3,
    Go = 4,
}

#[derive(Args, Debug)]
pub struct DownloadArgs {
    /// Optional field for the project path. If empty creates project folders in current working directory
    pub path: Option<String>,
    /// Specify the Advent of Code Day you want to download. A number between 1 and 25
    #[arg(short, long)]
    pub day: u32,
    #[arg(short, long, help=format!("Specify the Advent of Code year you want to download from. A number between 2015 and {}", get_latest_aoc_year()))]
    pub year: Option<u32>,

    /// Specify the language for the template generation.
    #[arg(short, long)]
    pub language: Option<Language>,
}

#[derive(Args, Debug)]
pub struct SubmitArgs {
    /// Your Advent of Code Solution. Expects a number
    pub solution: String,
    #[arg(short, long)]
    /// Specify the Advent of Code Day you want to submit. A number between 1 and 25
    pub day: u32,
    #[arg(short, long, help=format!("Specify the Advent of Code year you want to download from. A number between 2015 and {}", get_latest_aoc_year()))]
    pub year: u32,
    /// Specify the Advent of Code Part you want to submit. A number between 1 and 2
    #[arg(short, long)]
    pub part: u32,
}

#[derive(Args, Debug)]
pub struct CookieArgs {
    /// Optional field to set your session cookie. If empty prints your currently set cookie
    pub cookie: Option<String>,
}

pub fn check_args(args: &AocArgs) {
    let aoc_year = get_latest_aoc_year() as u32;

    match &args.command {
        Command::Download(a) => {
            match a.year {
                Some(year) => {
                    if year < 2015 || year > aoc_year {
                        println!(
                            "The Year argument has to be a number between 2015 and {}",
                            aoc_year
                        );
                        std::process::exit(1);
                    }
                }
                None => {}
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

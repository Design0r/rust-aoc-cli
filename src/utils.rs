use crate::args::DownloadArgs;
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use scraper::{Html, Selector};
use std::{env, path::PathBuf};
use tokio::fs;

pub const PY_TEMPLATE: &str = r#"from pathlib import Path

with open(Path(__file__).parent.parent / "inputs/REPLACE_DAY.txt") as f:
    file = f.read().strip()


def part_1() -> None:
    result = None
    print(f"Day REPLACE_DAY_NUM, Part 1: {result}")


def part_2() -> None:
    result = None
    print(f"Day REPLACE_DAY_NUM, Part 2: {result}")


if __name__ == "__main__":
    part_1()
    part_2()
"#;

pub const RS_TEMPLATE: &str = r#"use std::fs;

fn part_1(file: &String) {
    let result = 0;
    println!("Day REPLACE_DAY_NUM, Part 1: {}", result);
}

fn part_2(file: &String) {
    let result = 0;
    println!("Day REPLACE_DAY_NUM, Part 2: {}", result);
}

fn main() {
    let file = fs::read_to_string("../inputs/REPLACE_DAY.txt").expect("error reading file"); 
}
"#;

async fn create_dirs(base_path: &PathBuf) -> Result<()> {
    let input_path = base_path.join("inputs");
    let samples_path = base_path.join("samples");
    let src_path = base_path.join("src");

    let input_path_task = fs::create_dir_all(input_path);
    let samples_path_task = fs::create_dir_all(samples_path);
    let src_path_task = fs::create_dir_all(src_path);

    let results = tokio::join!(src_path_task, input_path_task, samples_path_task);

    results.0?;
    results.1?;
    results.2?;

    Ok(())
}

async fn create_files(base_path: &PathBuf, input: &String, args: &DownloadArgs) -> Result<()> {
    let file_template;
    let file_suffix = match base_path.join("Cargo.toml").is_file() {
        true => {
            file_template = RS_TEMPLATE;
            ".rs"
        }
        false => {
            file_template = PY_TEMPLATE;
            ".py"
        }
    };

    let left_pad = match args.day.to_string().len() {
        1 => String::from("0"),
        _ => String::from(""),
    };
    let day_fmt = format!("day_{}{}", left_pad, args.day);

    let input_file = base_path.join("inputs").join(day_fmt.clone() + ".txt");
    let samples_file = base_path.join("samples").join(day_fmt.clone() + ".txt");
    let src_file = base_path.join("src").join(day_fmt.clone() + file_suffix);
    let src_file_content = file_template
        .replace("REPLACE_DAY_NUM", &left_pad)
        .replace("REPLACE_DAY", &day_fmt);

    let input_file_task = fs::write(input_file, input);
    let samples_file_task = fs::write(samples_file, "");
    let py_file_task = fs::write(src_file, src_file_content);

    let results = tokio::join!(input_file_task, samples_file_task, py_file_task);

    results.0?;
    results.1?;
    results.2?;

    Ok(())
}

pub async fn scaffold_project(args: &DownloadArgs, input: &String) -> Result<()> {
    let base_path = match &args.path {
        Some(value) => PathBuf::from(value),
        None => env::current_dir()?,
    };

    create_dirs(&base_path).await?;
    create_files(&base_path, input, args).await?;

    Ok(())
}

pub fn get_latest_aoc_year() -> i32 {
    let local_time: DateTime<Local> = Local::now();
    let year = local_time.year();
    let month = local_time.month();

    if month == 12 {
        return year;
    }

    return year - 1;
}

pub fn get_article_content(html: String) -> Result<String> {
    let document = Html::parse_document(&html);
    let article_selector =
        Selector::parse("article").map_err(|e| anyhow!("Failed to parse selector: {}", e))?;

    let article_element = document.select(&article_selector).next();

    if let Some(element) = article_element {
        let content = element.text().collect::<Vec<_>>().join(" ");
        return Ok(content);
    } else {
        return Err(anyhow!("No article element found"));
    }
}

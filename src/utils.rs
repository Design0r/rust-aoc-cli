use std::{env, path::PathBuf};
use crate::args::DownloadArgs ;
use tokio;
use chrono::prelude::*;
use scraper::{Html, Selector};
use anyhow::{Result, anyhow};


pub const PY_TEMPLATE: &str = 
r#"from pathlib import Path

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

async fn create_dirs(input_path: PathBuf, samples_path: PathBuf) -> Result<()> {
    let input_path_task = tokio::fs::create_dir_all(input_path);
    let samples_path_task = tokio::fs::create_dir_all(samples_path);

    let (input_path_res, samples_path_res) = tokio::join!(input_path_task, samples_path_task);

    // Handle the results, returning an error if any operation failed.
    input_path_res.and_then(|_| samples_path_res)?;
    Ok(())
}

async fn create_files(input_file: PathBuf, samples_file: PathBuf, input: &String,  py_file: PathBuf, py_file_content: &str) -> Result<()> {
    let input_file_task = tokio::fs::write(input_file, input);
    let samples_file_task = tokio::fs::write(samples_file, "");
    let py_file_task = tokio::fs::write(py_file, py_file_content);

    let (input_file_res, samples_file_res, py_file_res) = tokio::join!(input_file_task, samples_file_task, py_file_task);

    input_file_res
        .and_then(|_| samples_file_res)
        .and_then(|_| py_file_res)?;
    
    Ok(())
}
pub async fn scaffold_project(args: &DownloadArgs, input: &String) -> Result<()> {
    let base_path = match &args.path {
        Some(value) => PathBuf::from(value),
        None => env::current_dir()?
    };

    let input_path = base_path.join("inputs");
    let samples_path = base_path.join("samples");

    let left_pad = match args.day.to_string().len() {
        1 => String::from("0"),
        _ => String::from("")
    };

    let day_fmt = format!("day_{}{}", left_pad, args.day);
    let intput_file = input_path.join(day_fmt.clone() + ".txt");
    let samples_file = samples_path.join(day_fmt.clone() + ".txt");
    let py_file = base_path.join(day_fmt.clone() + ".py");
    let py_file_content = PY_TEMPLATE.replace("REPLACE_DAY", &day_fmt).replace("REPLACE_DAY_NUM", &left_pad);

    let (dirs_result, files_result) = tokio::join!(create_dirs(input_path, samples_path), create_files(intput_file, samples_file, input, py_file, &py_file_content));

    dirs_result?;
    files_result?;

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

pub fn get_article_content(html: String) -> Result<String>{
    let document = Html::parse_document(&html);
    let article_selector = Selector::parse("article").map_err(|e| anyhow!("Failed to parse selector: {}", e))?;

    let article_element = document.select(&article_selector).next();

    if let Some(element) = article_element {
        let content = element.text().collect::<Vec<_>>().join(" ");
        return Ok(content);
    } else {
        return Err(anyhow!("No article element found"));
    }
}

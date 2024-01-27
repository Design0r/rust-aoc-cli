use std::{env, path::PathBuf};
use crate::args::DownloadArgs ;
use tokio;
use chrono::prelude::*;
use scraper::{Html, Selector};


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

pub async fn scaffold_project(args: &DownloadArgs, input: &String) {
    let base_path: PathBuf;
        match &args.path {
        Some(value) => base_path = PathBuf::from(value),
        None => base_path = env::current_dir().expect("Failed to get the current working directory"),
    }

    let input_path = base_path.join("inputs");
    let samples_path = base_path.join("samples");

    let left_pad;
    match args.day.to_string().len() {
        1 => left_pad = String::from("0"),
        2 => left_pad = String::from(""),
        _ => left_pad = String::from("")
    }
    let day_fmt = format!("day_{}{}", left_pad, args.day);
    let intput_file = input_path.join(day_fmt.clone() + ".txt");
    let samples_file = samples_path.join(day_fmt.clone() + ".txt");
    let py_file = base_path.join(day_fmt.clone() + ".py");
    let py_file_content = PY_TEMPLATE.replace("REPLACE_DAY", &day_fmt).replace("REPLACE_DAY_NUM", &left_pad);

    let create_dirs = async {
        let input_path_task = tokio::fs::create_dir_all(input_path);
        let samples_path_task = tokio::fs::create_dir_all(samples_path);

        let (input_path_res, samples_path_res) = tokio::join!(input_path_task, samples_path_task);

        input_path_res.expect("Error creating input folder structure");
        samples_path_res.expect("Error creating sample folder structure");
    };

    let create_files = async {
        let input_file_task = tokio::fs::write(intput_file, input);
        let samples_file_task = tokio::fs::write(samples_file, "");
        let py_file_task = tokio::fs::write(py_file, py_file_content);
        let (input_file_res, samples_file_res, py_file_res) = tokio::join!(input_file_task, samples_file_task, py_file_task);

        input_file_res.expect("Error creating input file structure");
        samples_file_res.expect("Error creating sample file structure");
        py_file_res.expect("Error creating sample file structure");
    };

    let ((), ()) = tokio::join!(create_dirs, create_files);
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

pub fn get_article_content(html: String) -> String {
    let document = Html::parse_document(&html);
    let article_selector = Selector::parse("article").unwrap();

    let content = document.select(&article_selector).last();

    return content.unwrap().text().collect::<Vec<_>>().join(" ");
}

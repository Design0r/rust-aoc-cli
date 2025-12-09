use crate::args::{DownloadArgs, Language};
use anyhow::{Result, anyhow};
use chrono::prelude::*;
use scraper::{Html, Selector};
use std::{env, fs, io::Write, path::PathBuf};

pub const PY_TEMPLATE: &str = r#"from pathlib import Path
from dataclasses import dataclass

from utils import benchmark

@dataclass(slots=True)
class Data:
    lines: list[str]


@benchmark
def parse() -> Data:
    with open(Path(__file__).parent.parent / "inputs/REPLACE_DAY.txt") as f:
        file = f.read().strip().splitlines()
    return Data(file)


@benchmark
def part_1(data: Data) -> None:
    result = 0
    print(f"Day REPLACE_DAY_NUM, Part 1: {result}")


@benchmark
def part_2(data: Data) -> None:
    result = 0
    print(f"Day REPLACE_DAY_NUM, Part 2: {result}")


if __name__ == "__main__":
    data = parse()
    part_1(data)
    part_2(data)
"#;

pub const RS_TEMPLATE: &str = r#"use std::{fs, time::Instant};

fn part_1(file: &Vec<&str>) {
    let mut result = 0;
    println!("Day REPLACE_DAY_NUM, Part 1: {}", result);
}

fn part_2(file: &Vec<&str>) {
    let mut result = 0;
    println!("Day REPLACE_DAY_NUM, Part 2: {}", result);
}

fn main() {
    let start = Instant::now();

    let file = fs::read_to_string("inputs/REPLACE_DAY.txt").expect("error reading file"); 
    let lines: Vec<&str> = file.lines().collect();
    part_1(&lines);
    part_2(&lines);

    println!("Finished in {}µs", start.elapsed().as_micros());
}
"#;

pub const CARGO_TEMPLATE: &str = r#"

[[bin]]
name = "REPLACE_DAY"
path = "src/REPLACE_DAY.rs""#;

pub const GO_TEMPLATE: &str = r#"package main

import (
	"fmt"
	"log"
    "os"
    "strings"
    "time"
)

type Data struct {
    Lines []string
}

func parse(path string) *Data {
	file, err := os.ReadFile(path)
	if err != nil {
		log.Fatalf("Error reading file %v", err)
	}
	stripped := strings.TrimSpace(string(file))
	lines := strings.Split(stripped, "\n")

	return &Data{Lines: lines}
}

func part1(data *Data) {
	result := 0
	fmt.Printf("Day REPLACE_DAY_NUM: Part 1: %v\n", result)
}

func part2(data *Data) {
	result := 0
	fmt.Printf("Day REPLACE_DAY_NUM: Part 2: %v\n", result)
}

func main() {
	fmt.Printf("------------------------------------\n")
	startTime := time.Now()
	lines := parse("inputs/REPLACE_DAY.txt")
	parseTime := time.Since(startTime)
	part1StartTime := time.Now()
	part1(lines)
	part1Time := time.Since(part1StartTime)
	part2StartTime := time.Now()
	part2(lines)
	part2Time := time.Since(part2StartTime)

	fmt.Printf("====================================\n")
	fmt.Printf("Finished Parsing in %v\n", parseTime)
	fmt.Printf("Finished Part 1 in %v\n", part1Time)
	fmt.Printf("Finished Part 2 in %v\n", part2Time)
	fmt.Printf("Total %v\n", time.Since(startTime))
	fmt.Printf("------------------------------------\n")
}

"#;

fn create_dirs(base_path: &PathBuf) -> Result<()> {
    let input_path = base_path.join("inputs");
    let samples_path = base_path.join("samples");
    let src_path = base_path.join("src");

    fs::create_dir_all(input_path)?;
    fs::create_dir_all(samples_path)?;
    fs::create_dir_all(src_path)?;

    println!("created folders");

    Ok(())
}

fn create_files(base_path: &PathBuf, input: &String, args: &DownloadArgs) -> Result<()> {
    let file_template;
    let file_suffix;

    match &args.language {
        Some(lang) => match lang {
            Language::Rust | Language::Rs => {
                file_template = RS_TEMPLATE;
                file_suffix = ".rs";
            }
            Language::Go => {
                file_template = GO_TEMPLATE;
                file_suffix = ".go";
            }
            Language::Python | Language::Py => {
                file_template = PY_TEMPLATE;
                file_suffix = ".py";
            }
        },
        None => {
            if base_path.join("Cargo.toml").is_file() {
                file_template = RS_TEMPLATE;
                file_suffix = ".rs";
            } else if base_path.join("go.mod").is_file() {
                file_template = GO_TEMPLATE;
                file_suffix = ".go";
            } else {
                file_template = PY_TEMPLATE;
                file_suffix = ".py";
            }
        }
    }

    let left_pad = match args.day.to_string().len() {
        1 => String::from("0"),
        _ => String::from(""),
    };
    let day_fmt = format!("day_{}{}", left_pad, args.day);
    let day_num = format!("{}{}", left_pad, args.day);

    let input_file = base_path.join("inputs").join(day_fmt.clone() + ".txt");
    let samples_file = base_path.join("samples").join(day_fmt.clone() + ".txt");
    let mut src_file = base_path.join("src").join(day_fmt.clone() + file_suffix);
    if file_suffix == ".go" {
        let _ = fs::create_dir(base_path.join("src").join(day_num.clone()));
        src_file = base_path
            .join("src")
            .join(&day_num)
            .join(day_fmt.clone() + file_suffix);
    } else if file_suffix == ".rs" {
        let mut cargo = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("Cargo.toml")
            .expect("Cargo.toml does nost exist, use cargo init to create one");

        let cargo_templ = CARGO_TEMPLATE.replace("REPLACE_DAY", &day_fmt);
        if let Err(e) = cargo.write_fmt(format_args!("{cargo_templ}")) {
            eprintln!("Failed to write to Cargo.toml: {e}");
        };
    }
    let src_file_content = file_template
        .replace("REPLACE_DAY_NUM", &day_num)
        .replace("REPLACE_DAY", &day_fmt);

    fs::write(input_file, input)?;
    fs::write(samples_file, "")?;
    fs::write(src_file, src_file_content)?;

    println!("created template files for {}", file_suffix);

    Ok(())
}

pub fn scaffold_project(args: &DownloadArgs, input: &String) -> Result<()> {
    let base_path = match &args.path {
        Some(value) => PathBuf::from(value),
        None => env::current_dir()?,
    };

    create_dirs(&base_path)?;
    create_files(&base_path, input, args)?;

    println!("completed template scaffolding");

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

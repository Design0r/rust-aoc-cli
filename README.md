# Advent of Code CLI

A Rust based Command Line Utility to download puzzles and input data and submit your solution for Advent of Code.

## Features
- Download puzzles and input data
- Automatic Project scaffolding for supported Languages (Based on detecting Cargo.toml or go.mod file, othwerwise chooses Python)
    - Rust
    - Python
    - Go
- Handles session cookie
- Directly submit your solution and check if your result is correct

## Installation

This command compiles and installs the "aoc" cli in the .cargo/bin directory

```shell
cargo install --path .
```

## Usage

1. Store your Advent of Code Session cookie

```shell
aoc cookie [COOKIE]

----------------------------------------------

Arguments:
  [COOKIE]

Options:
  -h, --help  Print help
```

2. Download a puzzle

```shell
aoc download [OPTIONS] --day <DAY> [PATH]

----------------------------------------------

Arguments:
  [PATH]  Optional field for the project path. If empty creates project folders in current working directory

Options:
  -d, --day <DAY>    Specify the Advent of Code Day you want to download. A number between 1 and 25
  -y, --year <YEAR>  Specify the Advent of Code year you want to download from. A number between 2015 and 2024
  -h, --help         Print help
```

3. Submit a solution

```shell
aoc submit --day <DAY> --year <YEAR> --part <PART> <SOLUTION>

----------------------------------------------

Arguments:
<SOLUTION> Your Advent of Code Solution. Expects a number

Options:
-d, --day <DAY> Specify the Advent of Code Day you want to submit. A number between 1 and 25
-y, --year <YEAR> Specify the Advent of Code year you want to download from. A number between 2015 and 2024
-p, --part <PART> Specify the Advent of Code Part you want to submit. A number between 1 and 2
-h, --help Print help
```

## Commands

```shell
aoc <COMMAND>

----------------------------------------------

Commands:
download Download an Advent of Code puzzle
submit Submit your Advent of Code solution
cookie Set your Advent of Code session cookie
help Print this message or the help of the given subcommand(s)

Options:
-h, --help Print help
-V, --version Print version
```

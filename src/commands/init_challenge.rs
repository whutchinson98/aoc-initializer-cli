use clap::Args;
use reqwest::Error;
use std::process::Command;
use std::{fs, path};

use crate::commands::config::load_config;

#[derive(Args)]
pub struct InitChallenge {
    /// The day of the challenge you want to initialize
    pub day: i32,
}

pub async fn init_challenge(day: &i32) -> Result<(), String> {
    println!("Initializing code challenge day {}", day);

    let config = match load_config() {
        Ok(config) => config,
        Err(e) => return Err(e.to_string()),
    };

    let aoc_session_id = config.aoc_key;
    let main_rs_content = include_str!("./templates/main");
    let gitignore_content = include_str!("./templates/gitignore");

    create_cargo_project(day);

    let res = get_input(&aoc_session_id, &day).await;

    if res.is_err() {
        panic!("Unable to get input from Advent of Code");
    }

    write_input_to_file(&day, &res.ok().unwrap());

    update_main_rs(&day, &main_rs_content);
    update_gitignore(&day, &gitignore_content);

    return Ok(());
}

fn create_cargo_project(day: &i32) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("cargo init q{day}", day = day))
        .output()
        .expect("error executing cargo init");
}

async fn get_input(aoc_session_id: &str, day: &i32) -> Result<String, Error> {
    let request_url = format!(
        "{request_url}{day}/input",
        request_url = "https://adventofcode.com/2022/day/",
        day = day
    );

    let client = reqwest::Client::new();

    let session_cookie = format!("session={aoc_session_id}", aoc_session_id = aoc_session_id);

    let response = client
        .get(&request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .await?;

    let res: String = response.text().await?.to_string();

    return Ok(res);
}

fn write_input_to_file(day: &i32, input: &str) {
    let path = format!("./q{day}/input.txt", day = day);
    let exists = path::Path::new(&path).exists();
    if exists {
        fs::remove_file(&path).expect("Unable to write file");
    }
    fs::write(&path, input).expect("Unable to write file");
}

fn update_main_rs(day: &i32, content: &str) {
    let main_rs = format!("./q{day}/src/main.rs", day = day);
    let exists = path::Path::new(&main_rs).exists();
    if exists {
        fs::remove_file(&main_rs).expect("Unable to write file");
    }
    fs::write(&main_rs, content).expect("Unable to write file");
}

fn update_gitignore(day: &i32, content: &str) {
    let gitignore = format!("./q{day}/.gitignore", day = day);
    let exists = path::Path::new(&gitignore).exists();
    if exists {
        fs::remove_file(&gitignore).expect("Unable to write file");
    }
    fs::write(&gitignore, content).expect("Unable to write file");
}

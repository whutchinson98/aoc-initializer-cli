use clap::Args;
use serde::{Deserialize, Serialize};
use std::{env, path, fs};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(debug_assertions)]
fn get_config_path() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap() + "/config.json",
        Err(_) => "FAILED".to_string(),
    }
}

#[cfg(not(debug_assertions))]
fn get_config_path() -> String {
    r#"~/.config/aoc-initializer/config.json"#.to_string()
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "aocKey")]
    pub aoc_key: String,
    #[serde(rename = "year")]
    pub year: i32,
}

pub fn load_config() -> Result<Config, String> {
    let config_file = get_config_path();
    let path = Path::new(&config_file);

    if !path.exists() {
        return Err(format!(
            "The config file was not able to be found at path {}",
            config_file
        ));
    }

    let mut file = match File::open(&config_file) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    };

    let config: Config = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    return Ok(config);
}

#[derive(Args)]
pub struct InitConfig {
    /// The AOC session key
    pub aoc_key: String,
    /// The year of AOC challenge
    pub year: i32,
}

pub fn init_config(aoc_key: &String, year: &i32) -> Result<(), String> {
    let template_config = include_str!("./templates/config");
    let template_config = &template_config.replacen("AOC_PLACEHOLDER", aoc_key, 1);
    let template_config = &template_config.replacen("YEAR_PLACEHOLDER", &year.to_string(), 1);

    println!("Initializing config file {}", template_config);

    let home = env::var("HOME").expect("You do not have a $HOME env var");

    let path_dir = home + "/.config/aoc-initializer";
    let path_config = path_dir.clone() + "/config.json";
    let exists = path::Path::new(&path_config).exists();
    if exists {
        fs::remove_file(&path_config).expect("Unable to write file");
    } else {
        fs::create_dir_all(&path_dir).expect("Unable to create config directory");
    }

    fs::write(&path_config, template_config).expect("Unable to write file");

    Ok(())
}

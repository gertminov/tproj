use chrono::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use toml::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let local_time = Local::now();
    let formatted_time = local_time.format("%Y-%m-%d %H-%M").to_string();
    let mut dir_name = formatted_time;
    if args.len() > 1 {
        dir_name = args[1].clone();
    }


    let toml = load_config();
    let base_dir_toml = toml["tempprojectdir"].as_str().unwrap();
    let base_dir = ensure_trailing_slash(base_dir_toml);
    let tproj_dir = base_dir + &dir_name;

    match fs::create_dir_all(tproj_dir.clone())
        .and_then(|_| fs::create_dir(tproj_dir.clone() + "/working"))
        .and_then(|_| fs::create_dir(tproj_dir.clone() + "/out"))
        .and_then(|_| {
            let terminal_args = format!("{}/working", tproj_dir.clone());
            println!("{}", terminal_args);
            Command::new("wt")
                .args(["-w 0 nt", "-d", &terminal_args])
                .status()
        }) {
        Ok(_) => println!("created dir"),
        Err(e) => println!("error creating dir: {}", e),
    }
}

fn ensure_trailing_slash(path: &str) -> String {
    let mut new_path = path.to_owned();
    if !new_path.ends_with("/") {
        new_path.push('/')
    }
    return new_path
}

fn load_config() -> Value{
    let mut file = File::open("config.toml").expect("failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file");
    let toml: Value = toml::from_str(&contents).expect("failed to parse TOML");
    return toml
}

use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

use toml::Value;

static MACOS_AGENT_CONFIG: &'static str = include_str!("./macos/heimsoft.tproj.plist");

pub fn load_config() -> Value {
    let config_path = get_config_path();
    if !config_path.exists() {
        create_config_file(&config_path);
    }
    let mut file = File::open(config_path).expect("failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read file");
    let toml: Value = toml::from_str(&contents).expect("failed to parse TOML");
    return toml;
}

pub fn init_config() {
    let config_path = get_config_path();
    if !config_path.exists() {
        create_config_file(&config_path);
    }
    register_autorun();
}

fn create_config_file(config_path: &PathBuf) {
    let config = dirs::home_dir().map_or_else(
        || "tempprojectdir = '' ".to_string(),
        |dir| format!("tempprojectdir = '{}'", dir.to_str().unwrap()),
    );

    fs::write(&config_path, config)
        .expect("could not write base config. please create a file names tproj.toml in the install directory");
}

fn get_config_path() -> PathBuf {
    return get_exe_location()
        .join("./tproj.toml")
        .to_owned();
}

fn get_exe_location() -> PathBuf {
    return env::current_exe()
        .expect("exe should exist")
        .canonicalize()
        .expect("error getting currentPath")
        .parent()
        .expect("the exe should be in a folder")
        .to_owned();
}

fn register_autorun() {
    if cfg!(target_os = "macos") {
        register_agent();
    } else if cfg!(target_os = "windows") {}
}

fn register_agent() {
    let exe_location = env::current_exe().unwrap().to_str().expect("could not get exe location").to_owned();
    let test = MACOS_AGENT_CONFIG.replace("{{exe_location}}", &exe_location);
    // get macos home dir
    let home_dir = simple_home_dir::home_dir().expect("could not get home dir");
    let plist_file = home_dir.join("Library/LaunchAgents/heimsoft.tproj.plist");

    // if plist_file.exists() { return; }

   let write_res =  File::create(&plist_file);
    match write_res {
        Ok(mut f) => {
            let write_res = f.write_all(test.as_bytes());
            if let Err(e) = write_res {
                println!("could not write agent file to {}: {}", &plist_file.to_string_lossy(), e);
                std::process::exit(1)
            }
        }
        Err(e) => {
            println!("could not write agent file to {}: {}", &plist_file.to_string_lossy(), e);
            std::process::exit(1)
        }
    }

    let res = Command::new("launchctl")
        .args(vec!["load", &*plist_file.to_string_lossy().to_string()])
        .output();
    match res {
        Ok(output) => {
            println!("{:?}", from_utf8(&output.stdout));
            if !output.status.success() {
                println!("could not load agent with launchd: {:?}", output.stderr);
                std::process::exit(1)
            }


        }
        Err(error) => {
            println!("could not load agent with launchd: {}", error);
            std::process::exit(1)
        }
    }
}

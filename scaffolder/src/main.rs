extern crate getopts;

use std::env;
use std::fs;
use std::path::PathBuf;

use chrono::prelude::*;
use random_str as random;

#[cfg(target_os = "windows")]
use win32console::console::WinConsole;

use crate::args::{InputArgs, parse_args};
use crate::cleaner::clean_old_projects;
use crate::config::load_config;
use crate::folder::open_folder;
use crate::open_latest::open_latest_folder;
use crate::terminal::read_terminal_command;

mod open_latest;
mod folder;
mod config;
mod terminal;
mod cleaner;
mod args;


fn main() {
    let args: Vec<String> = env::args().collect();
    let InputArgs { open_latest,
        open_explorer,
        open_both,
        clean,
        init,
        help,
        project_name } = parse_args(args);

    if help { return; }

    if init {
        config::init_config();
        return;
    }


    let toml = load_config();
    let base_dir_toml = toml["tempprojectdir"].as_str().unwrap();
    let base_path = PathBuf::from(base_dir_toml);

    if clean {
        clean_old_projects(&base_path);
        return;
    }


    let custom_terminal_cmd = toml.get("terminal").and_then(|t| t.as_array());
    let open_terminal_cmd = read_terminal_command(custom_terminal_cmd);

    if open_latest {
        open_latest_folder(open_explorer, open_both, &base_path, open_terminal_cmd);
        return;
    }

    let project_dir = get_new_dir_name(project_name, base_path);

    match fs::create_dir_all(&project_dir)
        .and_then(|_| fs::create_dir(project_dir.join( "working")))
        .and_then(|_| fs::create_dir(project_dir.join("out")))
        .and_then(|_| {
            let working_path = project_dir.join("working").to_str().expect("could not convert working dir pathbuff to string").to_string();
            open_folder(&working_path, open_both, open_explorer, open_terminal_cmd)
        }) {
        Ok(_) => println!("created dir"),
        Err(e) => println!("error creating dir: {}", e),
    }
}


fn ensure_unique_name(name: PathBuf) -> PathBuf {
    if fs::metadata(&name).is_ok() {
        name.join( random::get_string(2,
                                                true,
                                                false,
                                                false,
                                                false,
        ))
    } else {
        return name;
    }
}

fn get_new_dir_name(name: Option<String>, base_dir: PathBuf) -> PathBuf {
    return if let Some(custom_name) = name {
        let project_dir = base_dir.join(custom_name);
        ensure_unique_name(project_dir)
    } else {
        let local_time = Local::now();
        let project_dir = base_dir.join(local_time.format("%Y-%m-%d_%H-%M").to_string());
        ensure_unique_name(project_dir)
    };
}




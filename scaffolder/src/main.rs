use chrono::prelude::*;
use std::env;
use std::fs;
use std::fs::{File};
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use toml::Value;
use dirs;
use win32console::console::WinConsole;

extern crate getopts;

use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let local_time = Local::now();
    let formatted_time = local_time.format("%Y-%m-%d %H-%M").to_string();
    let mut dir_name = formatted_time;
    let program = args[0].clone();
    // if args.len() > 1 {
    //     dir_name = args[1].clone();
    // }

    let mut opts = Options::new();
    opts.optflag("e", "explorer", "opens explorer in newly created folder");
    opts.optflag("b", "both", "opens both new terminal window and explrer in created folder");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let open_explorer = matches.opt_present("e");
    let open_both = matches.opt_present("b");

    if !matches.free.is_empty() {
        dir_name = matches.free.first().unwrap().clone();
    }


    let toml = load_config();
    let base_dir_toml = toml["tempprojectdir"].as_str().unwrap();
    let base_dir = ensure_trailing_slash(base_dir_toml);
    let project_dir = base_dir + &dir_name;


    match fs::create_dir_all(&project_dir)
        .and_then(|_| fs::create_dir(project_dir.clone() + "/working"))
        .and_then(|_| fs::create_dir(project_dir.clone() + "/out"))
        .and_then(|_| {
            let windows_working_path = (project_dir.clone() + "/working").replace("/", "\\");
            let process_len = get_process_list_len();

            if process_len == 0 || open_explorer {
                start_explorer(&windows_working_path)
            } else if open_both {
                let res_terminal = start_terminal(&windows_working_path);
                let res_explorer = start_explorer(&windows_working_path);
                res_explorer.and(res_terminal)
            } else {
                start_terminal(&windows_working_path)
            }
        }) {
        Ok(_) => println!("created dir"),
        Err(e) => println!("error creating dir: {}", e),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [name] [options]", program);
    print!("{}", opts.usage(&brief))
}

fn start_explorer(path: &str) -> Result<ExitStatus, std::io::Error> {
    Command::new("explorer").args([path]).status()
}

fn start_terminal(path: &str) -> Result<ExitStatus, std::io::Error> {
    Command::new("wt")
        .args(["-w 0 nt", "-d", path])
        .status()
}

fn ensure_trailing_slash(path: &str) -> String {
    let mut new_path = path.to_owned();
    if !new_path.ends_with("/") {
        new_path.push('/')
    }
    return new_path;
}

fn get_process_list_len() -> u32 {
    let process_id = WinConsole::get_process_list().unwrap();
    process_id.get(1).unwrap().clone()
}

fn load_config() -> Value {
    let config_path = get_config_path();
    if !config_path.exists() {
        create_config_file(&config_path);
    }
    let mut file = File::open(config_path).expect("failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file");
    println!("contents: {}", &contents);
    let toml: Value = toml::from_str(&contents).expect("failed to parse TOML");
    return toml;
}

fn create_config_file(config_path: &PathBuf) {
    let config = dirs::home_dir()
        .map_or_else(
            || "tempprojectdir = '' ".to_string(),
            |dir| {
                format!("tempprojectdir = '{}'", dir.to_str().unwrap())
            },
        );

    println!("{}", &config_path.to_string_lossy());
    fs::write(&config_path, config)
        .expect("could not write base config. please create a file names tproj.toml in the install directory");
}


fn get_config_path() -> PathBuf {
    return env::current_exe()
        .expect("exe shoudl exist")
        .canonicalize()
        .expect("error getting currentPath")
        .parent()
        .expect("the exe should be in a folder")
        .join("../debug/tproj.toml")
        .to_owned();
}

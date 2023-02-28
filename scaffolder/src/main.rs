use chrono::prelude::*;
use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let local_time = Local::now();
    let formatted_time = local_time.format("%Y-%m-%d %H-%M").to_string();
    let mut dir_name = formatted_time;
    if args.len() > 1 {
        dir_name = args[1].clone();
    }
    let base_dir = "F:/tempProjects/";
    let tproj_dir = base_dir.to_owned() + &dir_name;

    match fs::create_dir(tproj_dir.clone())
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

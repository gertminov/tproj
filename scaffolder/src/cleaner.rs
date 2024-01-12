use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

use simplelog::{Config, LevelFilter, WriteLogger};

pub fn clean_old_projects(projects_dir: &PathBuf) {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(projects_dir.join("tproj.log"))
            .expect("Could not create log file")
    );
    let dirs_to_delete = get_projects_to_delete(projects_dir);
    delete_dirs(dirs_to_delete)
}


fn delete_dirs(dirs_to_delete: Vec<PathBuf>) {
    for dir in dirs_to_delete {
        match fs::remove_dir_all(&dir) {
            Ok(_) => {log::info!("Deleted {}", dir.to_string_lossy())}
            Err(e) => {
                log::error!("Could not delete {}: {}", dir.to_string_lossy(), e)
            }
        }
    }
}


fn get_projects_to_delete(projects_dir: &PathBuf) -> Vec<PathBuf> {
    if let Ok(entries) = fs::read_dir(projects_dir) {
        return entries.filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                let metadata = fs::metadata(&path).ok()?;
                if !metadata.is_dir() {
                    return None;
                }
                let deletable = fs::read_dir(&path).map(|proj_entries| {
                    let entries = proj_entries.filter(|pentry| {
                        match pentry {
                            Ok(pe) => {
                                if pe.file_name() == "out" {
                                    return check_for_empty_out(pe.path());
                                }
                                let hidden_file = pe.file_name().to_string_lossy().starts_with(".");
                                let working_dir = pe.file_name() == "working";
                                return !hidden_file && !working_dir;
                            }
                            Err(_) => { false }
                        }
                    }).count();
                    entries > 0
                });

                return deletable.ok().and_then(|d| {
                    if !d {
                        Some(path)
                    } else { None }
                });
            })
        }).collect::<Vec<PathBuf>>();
    }
    return vec![]
}

    fn check_for_empty_out(path: PathBuf) -> bool {
        let res = fs::read_dir(path).map(|entries| {
            entries.filter(|entry| {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    // Check if the file name does not start with a dot
                    !file_name.starts_with('.')
                } else {
                    false
                }
            }).count()
        });
        match res {
            Ok(c) => { c > 0 }
            Err(_) => { false }
        }
    }
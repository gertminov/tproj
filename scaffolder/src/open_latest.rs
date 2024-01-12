use std::fs;
use std::path::PathBuf;

use crate::folder::open_folder;
use crate::terminal::OpenTerminalCommand;

pub fn get_latest_project(base_dir: &PathBuf) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(base_dir){
        let mut files:Vec<_> = entries.filter_map(|entry| {
            entry.ok().and_then(|e|  {
                let path = e.path();
                let metadata = fs::metadata(&path).ok()?;
                if !metadata.is_dir() {
                    return None;
                }
                let create_time = metadata.created().ok()?;
                Some((path, create_time))

            })
        }).collect();

        files.sort_by(|a, b| b.1.cmp(&a.1));
        return files.first().map(|p| p.0.clone());
    }
    None

}

pub fn open_latest_folder(open_explorer: bool, open_both: bool, base_dir: &PathBuf, open_terminal_cmd: OpenTerminalCommand) {
    let latest_project = get_latest_project(base_dir)
        .map(|lproj| lproj.join("working"))
        .map(|proj_working| proj_working.to_str().map(|s| s.to_string()))
        .flatten();

    if let Some(project) = latest_project {
        open_folder(&project, open_both, open_explorer, open_terminal_cmd).expect("Could not open latest project");
    } else {
        println!("no projects found in {:?}", base_dir);
    }
}

use std::process::{Command, ExitStatus};

use crate::terminal::{OpenTerminalCommand, start_terminal};
#[cfg(target_os = "windows")]
use win32console::console::WinConsole;

pub fn open_folder(working_path: &str,
                   open_both: bool,
                   open_explorer: bool,
                   terminal_cmd: OpenTerminalCommand,
) -> Result<ExitStatus, std::io::Error> {
    if cfg!(windows) {
        let process_len = get_process_list_len();

        if process_len == 0 || open_explorer {
            start_explorer(working_path)
        } else if open_both {
            let res_terminal = start_terminal(terminal_cmd, working_path);
            let res_explorer = start_explorer(working_path);
            res_explorer.and(res_terminal)
        } else {
            start_terminal(terminal_cmd, working_path)
        }
    } else {
        if open_explorer {
            start_explorer(working_path)
        } else if open_both {
            let res_terminal = start_terminal(terminal_cmd, working_path);
            let res_explorer = start_explorer(working_path);
            res_explorer.and(res_terminal)
        } else {
            start_terminal(terminal_cmd, working_path)
        }
    }
}

fn start_explorer(path: &str) -> Result<ExitStatus, std::io::Error> {
    if cfg!(windows) {
        Command::new("explorer").args([path]).status()
    } else {
        Command::new("open").args([path]).status()
    }
}


fn get_process_list_len() -> u32 {
    if cfg!(target_os = "windows") {
        let process_id = WinConsole::get_process_list().unwrap();
        return process_id.get(1).unwrap().clone();
    } else {
        return 14;
    }
}

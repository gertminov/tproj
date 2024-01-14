use std::process::{Command, ExitStatus};
use toml::Value;

pub struct OpenTerminalCommand {
    pub command: String,
    pub args: Vec<String>,
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
pub fn read_terminal_command(input: Option<&Vec<Value>>) -> OpenTerminalCommand {
    return if let Some(vals) = input {
        let cmds: Vec<_> = vals
            .iter()
            .map(|v| {
                v.as_str()
                    .expect("malformed terminal arg string")
                    .to_string()
            })
            .collect();
        let cmd = cmds
            .first()
            .expect("Command to open terminal is misformed. please provide at least one command");
        OpenTerminalCommand {
            command: cmd.to_string(),
            args: cmds[1..].to_vec(),
        }
    } else {
        if cfg!(target_os = "windows") {
            OpenTerminalCommand {
                command: "wt".to_string(),
                args: vec_of_strings!["-w 0 nt", "-d"],
            }
        } else {
            OpenTerminalCommand {
                command: String::from("open"),
                args: vec_of_strings!["-a", "Terminal"],
            }
        }
    };
}

pub fn start_terminal(cmd: OpenTerminalCommand, path: &str) -> Result<ExitStatus, std::io::Error> {
    let args = vec![cmd.args, vec![path.to_string()]].concat();
    Command::new(cmd.command).args(args).status()
}

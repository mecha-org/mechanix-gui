use std::process::{Child, Command};

use anyhow::{bail, Result};

use crate::errors::{CommandError, CommandErrorCodes};
mod errors;

pub fn execute_command(command: String, args: Vec<String>) -> Result<bool> {
    let output = match Command::new(command).args(args).output() {
        Ok(output) => output,
        Err(e) => {
            bail!(CommandError::new(
                CommandErrorCodes::CommandExecuteError,
                format!("failed to execute command: {}", e),
            ))
        }
    };

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!(CommandError::new(
            CommandErrorCodes::CommandExecuteOutputError,
            format!("failed to get output from command: {}", error),
        ))
    }

    Ok(true)
}

pub fn spawn_command(command: String, args: Vec<String>) -> Result<Child> {
    println!("spawning command {:?} args {:?}", command, args);
    let child = match Command::new(command).args(args).spawn() {
        Ok(output) => output,
        Err(e) => {
            bail!(CommandError::new(
                CommandErrorCodes::CommandExecuteError,
                format!("failed to execute command: {}", e),
            ))
        }
    };

    Ok(child)
}

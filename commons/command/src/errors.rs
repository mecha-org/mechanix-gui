use std::fmt;

use tracing::error;

/// # Command Error Codes
///
/// Implements standard errors for the action bar
#[derive(Debug, Default, Clone, Copy)]
pub enum CommandErrorCodes {
    #[default]
    CommandExecuteError,
    CommandExecuteOutputError,
}

impl fmt::Display for CommandErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandErrorCodes::CommandExecuteError => {
                write!(f, "CommandExecuteError")
            }
            CommandErrorCodes::CommandExecuteOutputError => {
                write!(f, "CommandExecuteOutputError")
            }
        }
    }
}

/// # CommandError
///
/// Implements a standard error type for all action bar related errors
/// includes the error code (`CommandErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct CommandError {
    pub code: CommandErrorCodes,
    pub message: String,
}

impl CommandError {
    pub fn new(code: CommandErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

use std::fmt::{Debug, Display};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrintConfig {
    pub print_message: bool,
    pub append_message_to_session_log: bool,
    pub level: BevyLogLevel,
}

pub fn get_print_config(categories: Vec<LogCategory>, level: BevyLogLevel) -> PrintConfig {
    let mut print_message = false;
    let mut append_message_to_session_log = false;
    for category in categories {
        if LOG_CATEGORYS_TO_PRINT.contains(&category) {
            print_message = true;
        }
        if LOG_CATEGORYS_TO_APPEND_TO_SESSION_LOG.contains(&category) {
            append_message_to_session_log = true;
        }
        if print_message && append_message_to_session_log {
            break;
        }
    }

    PrintConfig {
        print_message,
        append_message_to_session_log,
        level,
    }
}

pub fn display_print_by_config<T: Display>(message_to_print: T, config: PrintConfig) {
    let log_message = format!("{}", message_to_print);
    print_by_config(log_message, config);
}

pub fn debug_print_by_config<T: Debug>(message_to_print: T, config: PrintConfig) {
    let log_message = format!("{:?}", message_to_print);
    print_by_config(log_message, config);
}

fn print_by_config(log_message: String, config: PrintConfig) {
    if config.print_message {
        match config.level {
            BevyLogLevel::Error => error!(log_message),
            BevyLogLevel::Warning => warn!(log_message),
            BevyLogLevel::Info => info!(log_message),
        }
    }
    if config.append_message_to_session_log {
        append_to_game_session_log_file(log_message);
    }
}

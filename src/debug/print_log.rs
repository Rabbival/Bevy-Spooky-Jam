use crate::prelude::*;
use std::fmt::Display;

fn print_log<T: Display>(message: T, categories: Vec<LogCategory>, level: BevyLogLevel) {
    if !LOG_LEVELS_TO_PRINT.contains(&level) {
        return;
    }
    let print_config = get_print_config(categories, level);
    display_print_by_config(message, print_config);
}

pub fn print_error<T: Display>(message: T, log_categories: Vec<LogCategory>) {
    print_log(message, log_categories, BevyLogLevel::Error);
}

pub fn print_warning<T: Display>(message: T, log_categories: Vec<LogCategory>) {
    print_log(message, log_categories, BevyLogLevel::Warning);
}

pub fn print_info<T: Display>(message: T, log_categories: Vec<LogCategory>) {
    print_log(message, log_categories, BevyLogLevel::Info);
}

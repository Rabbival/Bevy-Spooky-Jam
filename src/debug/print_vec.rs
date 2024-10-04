use std::fmt::Debug;

use crate::prelude::*;

fn print_vec<T: Debug>(
    message: &str,
    vec: &Vec<T>,
    categories: Vec<LogCategory>,
    level: BevyLogLevel,
) {
    if !LOG_LEVELS_TO_PRINT.contains(&level) {
        return;
    }
    let print_config = get_print_config(categories, level);
    let full_message_to_print = String::from(message) + &stringify_vector(vec);
    display_print_by_config(full_message_to_print, print_config);
}

fn stringify_vector<T: Debug>(vec: &Vec<T>) -> String {
    let mut stringified_vector = String::from("[ ");
    for element in vec {
        stringified_vector += &format!("{:?}, ", element);
    }
    if !vec.is_empty() {
        stringified_vector.pop();
        stringified_vector.pop();
        stringified_vector.push(' ');
    }
    stringified_vector.push(']');
    stringified_vector
}

pub fn print_error_vec<T: Debug>(message: &str, vec: &Vec<T>, log_categories: Vec<LogCategory>) {
    print_vec(message, vec, log_categories, BevyLogLevel::Error);
}

pub fn print_warning_vec<T: Debug>(message: &str, vec: &Vec<T>, log_categories: Vec<LogCategory>) {
    print_vec(message, vec, log_categories, BevyLogLevel::Warning);
}

pub fn print_info_vec<T: Debug>(message: &str, vec: &Vec<T>, log_categories: Vec<LogCategory>) {
    print_vec(message, vec, log_categories, BevyLogLevel::Info);
}

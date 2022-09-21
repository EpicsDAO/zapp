use crate::constants::*;
use console::style;

pub fn log_success(text: &str) {
    println!("{}{}", COMPLETE_EMOJI, style(text).white().bold());
}

pub fn log_error(text: &str) {
    println!("{}{}", ERROR_EMOJI, style(text).white().bold());
}

pub fn log_new(text: &str) {
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}",
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        LAUNCH_EMOJI,
        style(text).white().bold()
    );
}

pub fn log_white(text: &str) {
    println!("{}", style(text).blue().bold());
}

pub fn log_input(text: &str) {
    println!("{}{}", PAPER_EMOJI, style(text).white().bold());
}

pub fn log_time(text: &str) {
    println!("{}{}", TIME_EMOJI, style(text).white().bold());
}

pub fn log_create_file(text: &str) {
    println!("{}{}", FILE_EMOJI, style(text).white().bold());
}

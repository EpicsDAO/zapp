use console::style;
use crate::constants::*;

pub async fn log_success(text: &str) {
  println!(
      "{}{}",
      COMPLETE_EMOJI,
      style(text).white().bold()
  );
}

pub async fn log_error(text: &str) {
  println!(
      "{}{}",
      ERROR_EMOJI,
      style(text).white().bold()
  );
}

pub async fn log_input(text: &str) {
  println!(
      "{}{}",
      PAPER_EMOJI,
      style(text).white().bold()
  );
}

pub async fn log_create_file(text: &str) {
  println!(
      "{}{}",
      FILE_EMOJI,
      style(text).white().bold()
  );
}


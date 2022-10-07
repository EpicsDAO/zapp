use crate::new::generation::populate_working_dir;
use crate::style_print::{log_new, log_white};
use std::path::Path;

mod generation;

pub fn handle_new(app_name: &str, gen_path: &Path) {
    populate_working_dir(&gen_path, &app_name);
    endroll(&app_name);
}

fn endroll(app_name: &str) {
    let text1 = "  ███████╗ █████╗ ██████╗ ██████╗ ";
    let text2 = "  ╚══███╔╝██╔══██╗██╔══██╗██╔══██╗";
    let text3 = "    ███╔╝ ███████║██████╔╝██████╔╝";
    let text4 = "   ███╔╝  ██╔══██║██╔═══╝ ██╔═══╝ ";
    let text5 = "  ███████╗██║  ██║██║     ██║     ";
    let text6 = "  ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝     ";
    log_white(text1);
    log_white(text2);
    log_white(text3);
    log_white(text4);
    log_white(text5);
    log_white(text6);
    log_new(&format!("\nRust Serverless Framework\n$ cd {app_name}\n$ zapp docker psql\n$ cargo run\n\nGo to: http://localhost:3000/api/graphql"));
}

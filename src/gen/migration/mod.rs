use crate::gen::migration::{creation::create_migration, registration::register_migration};
use crate::style_print::log_error;
use chrono::NaiveDateTime;
use std::fs;
use std::path::Path;

mod creation;
mod registration;

pub(in crate::gen) fn process_migration(model: &str, dt: NaiveDateTime, gen_path: &Path) {
    let migration_src_dir = gen_path.join("migration").join("src");

    fs::create_dir_all(migration_src_dir.as_path()).unwrap_or_else(|why| {
        log_error(&format!("! {:?}", why.kind()));
    });

    create_migration(model, dt, &migration_src_dir);
    register_migration(model, &migration_src_dir);
}

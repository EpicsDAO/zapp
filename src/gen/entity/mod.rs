use crate::gen::entity::{creation::create_entity, registration::register_entity};
use crate::style_print::log_error;
use std::fs;
use std::path::Path;

mod creation;
mod registration;

pub(in crate::gen) fn process_entity(model: &str, gen_path: &Path) {
    let entity_src_dir = gen_path.join("entity").join("src");

    fs::create_dir_all(entity_src_dir.as_path()).unwrap_or_else(|why| {
        log_error(&format!("! {:?}", why.kind()));
    });

    create_entity(model, &entity_src_dir);
    register_entity(model, &entity_src_dir);
}

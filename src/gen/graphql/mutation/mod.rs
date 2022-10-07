use crate::gen::graphql::mutation::{creation::create_mutation, registration::register_mutation};
use crate::style_print::log_error;
use std::fs;
use std::path::Path;

mod creation;
mod registration;

pub(in crate::gen) fn process_graphql_mutation(model: &str, gen_path: &Path) {
    let mutation_dir = gen_path.join("src").join("graphql").join("mutation");

    fs::create_dir_all(mutation_dir.as_path()).unwrap_or_else(|why| {
        log_error(&format!("! {:?}", why.kind()));
    });

    create_mutation(model, &mutation_dir);
    register_mutation(model, &mutation_dir);
}

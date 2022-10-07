use crate::gen::graphql::query::{creation::create_query, registration::register_query};
use crate::style_print::log_error;
use std::fs;
use std::path::Path;

mod creation;
mod registration;

pub(in crate::gen) fn process_graphql_query(model: &str, gen_path: &Path) {
    let query_dir = gen_path.join("src").join("graphql").join("query");

    fs::create_dir_all(query_dir.as_path()).unwrap_or_else(|why| {
        log_error(&format!("! {:?}", why.kind()));
    });

    create_query(model, &query_dir);
    register_query(model, &query_dir);
}

use std::{fs, io};
use std::path::Path;
use convert_case::{Case, Casing};
use crate::g::entity::process_entity;
use crate::g::graphql::mutation::process_graphql_mutation;
use crate::g::graphql::query::process_graphql_query;
use crate::g::migration::process_migration;

pub mod entity;
pub mod migration;
pub mod graphql;


pub(self) async fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}


pub(self) fn to_upper_camel(s: &str) -> String {
    s.to_case(Case::UpperCamel)
}

pub async fn process_g(model: &str, gen_path: &Path) {
    process_entity(model, gen_path).await;
    process_migration(model, gen_path).await;
    process_graphql_query(model, gen_path).await;
    process_graphql_mutation(model, gen_path).await;
}




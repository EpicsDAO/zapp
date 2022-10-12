mod mutation_dir;
mod query_dir;

use crate::new::generation::src_dir::graphql::mutation_dir::populate_mutation_dir;
use crate::new::generation::src_dir::graphql::query_dir::populate_query_dir;
use crate::template_file;
use std::fs;
use std::io::Write;
use std::path::Path;
pub fn populate_graphql_dir(gen_path: &Path, app_name: &str) {
    let graphql_dir = gen_path.join("graphql");
    fs::create_dir(&graphql_dir).unwrap();

    populate_mutation_dir(&graphql_dir, &app_name);
    populate_query_dir(&graphql_dir, &app_name);

    template_file!(
        "../../../../../resources/templates/new/src/graphql/mod.rs",
        &graphql_dir.join("mod.rs")
    );
    template_file!(
        "../../../../../resources/templates/new/src/graphql/schema.rs",
        &graphql_dir.join("schema.rs")
    );
}

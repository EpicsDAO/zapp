use crate::template_file;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn populate_query_dir(gen_path: &Path, _app_name: &str) {
    let query_dir = gen_path.join("query");
    fs::create_dir(&query_dir).unwrap();

    template_file!(
        "../../../../../../resources/templates/new/src/graphql/query/mod.rs",
        &query_dir.join("mod.rs")
    );
}

use crate::template_file;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn populate_mutation_dir(gen_path: &Path, _app_name: &str) {
    let mutation_dir = gen_path.join("mutation");
    fs::create_dir(&mutation_dir).unwrap();

    template_file!(
        "../../../../../../resources/templates/new/src/graphql/mutation/common.rs",
        &mutation_dir.join("common.rs")
    );
    template_file!(
        "../../../../../../resources/templates/new/src/graphql/mutation/mod.rs",
        &mutation_dir.join("mod.rs")
    );
}

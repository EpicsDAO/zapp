mod graphql;

use crate::new::generation::src_dir::graphql::populate_graphql_dir;
use crate::template_file;
use std::fs;
use std::io::Write;
use std::path::Path;
pub(super) fn populate_src_dir(gen_path: &Path, app_name: &str) {
    let src_dir = gen_path.join("src");
    fs::create_dir(&src_dir).unwrap();

    populate_graphql_dir(&src_dir, &app_name);

    template_file!(
        "../../../../resources/templates/new/src/main.rs",
        &src_dir.join("main.rs")
    );
    template_file!(
        "../../../../resources/templates/new/src/db.rs",
        &src_dir.join("db.rs")
    );
}

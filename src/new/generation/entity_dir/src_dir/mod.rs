use crate::template_file;
use std::fs;
use std::io::Write;
use std::path::Path;

pub(super) fn populate_src_dir(gen_path: &Path, _app_name: &str) {
    let src_dir = gen_path.join("src");
    fs::create_dir(&src_dir).unwrap();

    template_file!(
        "../../../../../resources/templates/new/entity/src/lib.rs",
        &src_dir.join("lib.rs")
    );
}

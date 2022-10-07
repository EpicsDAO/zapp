mod src_dir;

use crate::new::generation::entity_dir::src_dir::populate_src_dir;
use crate::template_file;
use std::fs;
use std::io::Write;
use std::path::Path;

pub(super) fn populate_entity_dir(gen_path: &Path, app_name: &str) {
    let entity_dir = gen_path.join("entity");
    fs::create_dir(&entity_dir).unwrap();

    populate_src_dir(&entity_dir, &app_name);

    template_file!(
        "../../../../resources/templates/new/entity/Cargo.toml",
        &entity_dir.join("Cargo.toml")
    );
}

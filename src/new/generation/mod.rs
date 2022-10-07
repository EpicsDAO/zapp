use crate::new::generation::entity_dir::populate_entity_dir;
use crate::new::generation::migration_dir::populate_migration_dir;
use crate::new::generation::src_dir::populate_src_dir;
use std::io::Write;
use std::path::Path;
use std::{fs, str};

mod entity_dir;
mod migration_dir;
mod src_dir;

#[macro_export]
macro_rules! template_file {
    ($source: literal, &$target: expr) => {
        let template_fn = std::convert::identity::<String>;
        template_file!($source, $target, template_fn);
    };
    ($source: literal, $target:expr, $template_fn: expr) => {
        let template = include_str!($source).to_string();
        let templated_file = $template_fn(template);
        let mut file = fs::File::create($target).unwrap();
        file.write_all(templated_file.as_bytes()).unwrap();
    };
}

pub(super) fn populate_working_dir(gen_path: &Path, app_name: &str) {
    let working_dir = gen_path.join(app_name);
    fs::create_dir(&working_dir).unwrap();

    populate_entity_dir(&working_dir, &app_name);
    populate_migration_dir(&working_dir, &app_name);
    populate_src_dir(&working_dir, &app_name);

    template_file!(
        "../../../resources/templates/new/.dockerignore",
        &working_dir.join(".dockerignore")
    );

    template_file!(
        "../../../resources/templates/new/.env",
        &working_dir.join(".env")
    );

    template_file!(
        "../../../resources/templates/new/.gitignore",
        &working_dir.join(".gitignore"),
        |gitignore: String| gitignore.replace("{{ app_name }}", app_name)
    );

    template_file!(
        "../../../resources/templates/new/Cargo.toml",
        &working_dir.join("Cargo.toml"),
        |cargo_toml: String| cargo_toml.replace("{{ app_name }}", app_name)
    );

    template_file!(
        "../../../resources/templates/new/Dockerfile",
        &working_dir.join("Dockerfile"),
        |dockerfile: String| { dockerfile.replace("{{ app_name }}", &app_name) }
    );

    template_file!(
        "../../../resources/templates/new/README.md",
        working_dir.join("README.md"),
        |readme: String| readme.replace("{{ app_name }}", app_name)
    );
}

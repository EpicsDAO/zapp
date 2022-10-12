use sea_orm_cli::run_migrate_init;
use std::fs;
use std::path::Path;

pub fn populate_migration_dir(gen_path: &Path, _app_name: &str) {
    let working_dir = gen_path.join("migration");
    fs::create_dir(&working_dir).unwrap();

    // use the sea-orm-cli to generate the migration directory programmatically
    run_migrate_init(working_dir.to_str().unwrap()).expect("Could not init migration!");
    // FIXME(@JonasCir) we need a couple of fixes, maybe we can upstream some of them to sea-orm-cli?

    // 1. We need to remove the example migration file: Find a file which ends in _create_table.rs
    // and remove it

    let migration_src_dir = &working_dir.join("src");
    let initial_migration_file = fs::read_dir(&migration_src_dir)
        .unwrap()
        .map(|f| f.unwrap().file_name())
        .find(|name| name.to_str().unwrap().ends_with("_create_table.rs"))
        .unwrap();

    let buf = &migration_src_dir.join(&initial_migration_file);
    fs::remove_file(buf).unwrap();

    // 2. We need to patch the lib.rs file to remove the example migration

    let mod_name = initial_migration_file.to_str().unwrap().replace(".rs", "");
    let lib_rs_path = &migration_src_dir.join("lib.rs");

    let mut lib_rs = fs::read_to_string(lib_rs_path).unwrap();
    lib_rs = lib_rs.replace(&format!("mod {};", mod_name), "");
    lib_rs = lib_rs.replace(
        &format!("vec![Box::new({}::Migration)]", mod_name),
        "vec![]",
    );

    // remove lines such that cargo fmt is happy
    let mut tmp = lib_rs.lines().collect::<Vec<_>>();
    tmp.remove(1);
    tmp.remove(2);
    lib_rs = tmp.join("\n");
    lib_rs.push('\n');
    fs::write(lib_rs_path, lib_rs).unwrap();

    // 3. We need to patch the cargo.toml file

    let cargo_toml_path = &working_dir.join("Cargo.toml");
    let cargo_toml = fs::read_to_string(cargo_toml_path).unwrap();
    let mut cargo_toml = cargo_toml.lines().take(11).collect::<Vec<_>>().join("\n");
    cargo_toml.push_str("\n");
    cargo_toml.push_str("async-std = {workspace = true}\n");
    cargo_toml.push_str("sea-orm = {workspace = true}\n");
    cargo_toml.push_str("sea-orm-migration = {workspace = true}\n");

    fs::write(cargo_toml_path, cargo_toml).unwrap();
}

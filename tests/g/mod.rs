use std::{fs::File, io::Read, path::Path};

mod integration_test;

fn assert_file_equality(resource_dir: &Path, tmp_dir: &Path, path: &str) {
    let mut resource_file = File::open(resource_dir.join(path)).unwrap();
    let mut tmp_file = File::open(tmp_dir.join(path)).unwrap();

    let mut buf1 = String::new();
    let mut buf2 = String::new();

    resource_file.read_to_string(&mut buf1).unwrap();
    tmp_file.read_to_string(&mut buf2).unwrap();
    assert_eq!(buf1, buf2);
}

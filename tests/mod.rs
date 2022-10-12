use itertools::Itertools;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tempdir::TempDir;
use walkdir::WalkDir;

mod g;
mod new;

fn assert_directory_equality(
    tmp_dir_name: &str,
    test_subdirectory: &str,
    test_resources: &str,
    handler: &dyn Fn(&TempDir) -> (),
) {
    // create a fresh temp directory for the test
    let tmp_dir = TempDir::new(tmp_dir_name).unwrap();
    // load the resources directory which contains the expected output
    let resource_dir = current_dir().unwrap().join(test_resources);

    // invoke the handler to generate the output
    handler(&tmp_dir);

    // craft the directory we generated the output to
    let under_test_dir = tmp_dir.path().join(test_subdirectory);

    // check that both directories contain the same number files
    assert_eq!(
        WalkDir::new(&resource_dir).into_iter().count(),
        WalkDir::new(&under_test_dir).into_iter().count()
    );

    // ensure traversal order is the same
    let resource_iter = WalkDir::new(&resource_dir)
        .into_iter()
        .map(Result::unwrap)
        .sorted_by(|f, s| Ord::cmp(f.path(), s.path()));

    let tmp_iter = WalkDir::new(&under_test_dir)
        .into_iter()
        .map(Result::unwrap)
        .sorted_by(|f, s| Ord::cmp(f.path(), s.path()));

    // above check ensures that both iterators have the same length, so we can zip it
    // check that both directories contain the same files
    resource_iter
        .zip(tmp_iter)
        .for_each(|(resource_entry, tmp_entry)| {
            assert_file_equality(resource_entry.path(), tmp_entry.path());
        });
}

fn assert_file_equality(resource_file: &Path, test_file: &Path) {
    // ensure both files are named the same
    assert_eq!(resource_file.file_name(), test_file.file_name());

    let res_meta = fs::metadata(resource_file).unwrap();
    let test_meta = fs::metadata(test_file).unwrap();
    let both_dirs = res_meta.is_dir() && test_meta.is_dir();
    // ensure that both files are either files or directories
    assert!(both_dirs || (!res_meta.is_dir() && !test_meta.is_dir()));

    if both_dirs {
        // we have two directories and we checked that all are named the same
        return;
    }

    // ensure that both files have the same content

    let mut resource_file = File::open(resource_file).unwrap();
    let mut tmp_file = File::open(test_file).unwrap();

    let mut expected = String::new();
    let mut actual = String::new();

    resource_file.read_to_string(&mut expected).unwrap();
    tmp_file.read_to_string(&mut actual).unwrap();

    assert_eq!(actual, expected);
}

use crate::assert_directory_equality;
use tempdir::TempDir;
use zapp::new::handle_new;

#[test]
fn new_test_project() {
    let app_name = "new_test_project";
    assert_directory_equality(
        "zapp-new-integration-tests",
        app_name,
        "tests/new/resources/new_test_project",
        &|tmp_dir: &TempDir| handle_new(app_name, tmp_dir.path()),
    );
}

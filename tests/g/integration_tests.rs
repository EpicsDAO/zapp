use crate::assert_directory_equality;
use chrono::NaiveDate;
use tempdir::TempDir;
use zapp::g::handle_g;

#[test]
fn gen_one_user_model() {
    // we need to create a dedicated sub-directory in the temp folder where we generate the output to
    let test_name = "gen_one_user_model";

    assert_directory_equality(
        "zapp-g-integration-tests",
        test_name,
        "tests/g/resources/gen_one_user_model",
        &|tmp_dir: &TempDir| {
            handle_g(
                "user",
                NaiveDate::from_ymd(2022, 7, 16).and_hms(23, 39, 33),
                tmp_dir.path().join(test_name).as_path(),
            )
        },
    );
}

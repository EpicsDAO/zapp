use crate::gen::assert_file_equality;
use chrono::NaiveDate;
use std::path::Path;
use tempdir::TempDir;
use zapp::gen::handle_gen;

#[test]
fn gen_one_user_model() {
    let tmp_dir = TempDir::new("zapp-gen-integration-tests").unwrap();
    let resource_dir = Path::new("tests/g/resources/gen_one_user_model");

    let test_dt = NaiveDate::from_ymd(2022, 7, 16).and_hms(23, 39, 33);

    handle_gen("user", test_dt, tmp_dir.path());

    assert_file_equality(resource_dir, tmp_dir.path(), "entity/src/user.rs");
    assert_file_equality(resource_dir, tmp_dir.path(), "entity/src/lib.rs");

    assert_file_equality(
        resource_dir,
        tmp_dir.path(),
        "migration/src/m20220716_233933_create_user_table.rs",
    );
    assert_file_equality(resource_dir, tmp_dir.path(), "migration/src/lib.rs");

    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/query/user.rs");
    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/query/gen");

    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/mutation/user.rs");
    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/mutation/gen");
}

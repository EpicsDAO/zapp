use std::path::Path;
use chrono::NaiveDate;
use tempdir::TempDir;
use zapp::g::process_g;
use tokio::test;
use crate::g::assert_file_equality;

#[test]
async fn gen_one_user_model() {
    let tmp_dir = TempDir::new("zapp-g-integration-tests").unwrap();
    let resource_dir = Path::new("tests/g/resources/gen_one_user_model");

    let test_dt = NaiveDate::from_ymd(2022, 7, 16).and_hms(23, 39, 33);

    process_g("user", test_dt, tmp_dir.path()).await;

    assert_file_equality(resource_dir, tmp_dir.path(), "entity/src/user.rs");
    assert_file_equality(resource_dir, tmp_dir.path(), "entity/src/lib.rs");

    assert_file_equality(resource_dir, tmp_dir.path(), "migration/src/m20220716_233933_create_user_table.rs");
    assert_file_equality(resource_dir, tmp_dir.path(), "migration/src/lib.rs");

    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/query/user.rs");
    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/query/mod.rs");

    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/mutation/user.rs");
    assert_file_equality(resource_dir, tmp_dir.path(), "src/graphql/mutation/mod.rs");
}



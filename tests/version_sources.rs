use php2shell_checker::infrastructure::parsers::collect_versions;
use std::fs;

#[test]
fn reads_php_version_sources() {
    let root = std::env::temp_dir().join(format!("php2shell-test-{}", std::process::id()));
    fs::create_dir_all(&root).expect("create test directory");
    fs::write(root.join(".php-version"), "8.2.34\n").expect("write php version");
    let versions = collect_versions(&root).expect("parse version source");
    assert_eq!(versions.len(), 1);
    assert!(
        matches!(versions[0].value, php2shell_checker::domain::VersionValue::Exact(ref value) if value == "8.2.34")
    );
    let _ = fs::remove_file(root.join(".php-version"));
    let _ = fs::remove_dir(root);
}

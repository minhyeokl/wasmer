// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build.

#[test]
fn test_unstable_path_symlink() {
    assert_wasi_output!(
        "../wasi_test_resources/unstable/path_symlink.wasm",
        "unstable_path_symlink",
        vec![],
        vec![
            (
                "temp".to_string(),
                ::std::path::PathBuf::from("tests/wasi_test_resources/test_fs/temp")
            ),
            (
                "hamlet".to_string(),
                ::std::path::PathBuf::from("tests/wasi_test_resources/test_fs/hamlet")
            ),
        ],
        vec![],
        "../wasi_test_resources/path_symlink.out"
    );
}

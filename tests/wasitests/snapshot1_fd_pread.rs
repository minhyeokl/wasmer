// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build.

#[test]
#[ignore]
fn test_snapshot1_fd_pread() {
    assert_wasi_output!(
        "../wasi_test_resources/snapshot1/fd_pread.wasm",
        "snapshot1_fd_pread",
        vec![],
        vec![(
            ".".to_string(),
            ::std::path::PathBuf::from("tests/wasi_test_resources/test_fs/hamlet")
        ),],
        vec![],
        "../wasi_test_resources/fd_pread.out"
    );
}

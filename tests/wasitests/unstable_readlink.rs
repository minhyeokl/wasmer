// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build.

#[test]
fn test_unstable_readlink() {
    assert_wasi_output!(
        "../wasi_test_resources/unstable/readlink.wasm",
        "unstable_readlink",
        vec![],
        vec![(
            ".".to_string(),
            ::std::path::PathBuf::from("tests/wasi_test_resources/test_fs/hamlet")
        ),],
        vec![],
        "../wasi_test_resources/readlink.out"
    );
}

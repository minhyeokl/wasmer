// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build.

#[test]
fn test_snapshot1_envvar() {
    assert_wasi_output!(
        "../wasi_test_resources/snapshot1/envvar.wasm",
        "snapshot1_envvar",
        vec![],
        vec![],
        vec!["DOG=1".to_string(), "CAT=2".to_string(),],
        "../wasi_test_resources/envvar.out"
    );
}

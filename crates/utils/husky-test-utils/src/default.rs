use crate::*;

pub fn default_test_dirs() -> Vec<PathBuf> {
    let env = HuskyDevPathEnv::new();
    vec![
        env.lang_dev_library_dir().to_owned(),
        env.lang_dev_examples_dir().to_owned(),
    ]
}

pub fn default_expect_test_base_outs() -> Vec<(PathBuf, PathBuf)> {
    let env = HuskyDevPathEnv::new();
    vec![
        (
            env.lang_dev_library_dir().to_owned(),
            env.cargo_manifest_dir().join("expect-files/library"),
        ),
        (
            env.lang_dev_examples_dir().to_owned(),
            env.cargo_manifest_dir().join("expect-files/examples"),
        ),
    ]
}
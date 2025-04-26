
#[macro_export]
macro_rules! get_git_default_branch_name {
    () => {
        mtc_toolbelt::cmd::get_cmd_out("git".into(), vec!["var".to_string(), "GIT_DEFAULT_BRANCH".to_string()]).unwrap()
    };
}
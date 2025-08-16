use assert_cmd::Command;

#[test]
fn runs_help() {
    let mut cmd = Command::cargo_bin("todo_cli").unwrap();
    cmd.arg("--help").assert().success();
}
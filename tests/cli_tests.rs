use std::process::Command;

#[test]
fn test_cli_test1() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/test1.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "35");
}

#[test]
fn test_cli_test2() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/test2.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello world");
}

#[test]
fn test_cli_test3() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/test3.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "-10");
}

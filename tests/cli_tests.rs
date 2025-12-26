use std::process::Command;

#[test]
fn test_cli_recursion() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/recursion.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "55");
}

#[test]
fn test_cli_math() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/math.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "22.5");
}

#[test]
fn test_cli_strings() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/strings.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "Hello, Toy User!");
}

#[test]
fn test_cli_control_flow() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/control_flow.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "small medium large");
}

#[test]
fn test_cli_assignment() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/assignment.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "248.5");
}

#[test]
fn test_cli_comments() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "tests/comments.toy"])
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "30");
}

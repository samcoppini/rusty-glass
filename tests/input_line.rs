mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn input_line_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        (_i)l.?(_o)o.?
        (_i)l.?(_o)o.?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .write_stdin("ab\ncd\nef")
       .assert()
       .success()
       .stdout("ab\ncd\n");

    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .write_stdin("ab\ncd\n")
        .assert()
        .success()
        .stdout("ab\ncd\n");

    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .write_stdin("ab\ncd")
        .assert()
        .success()
        .stdout("ab\ncd");

    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .write_stdin("ab\n")
        .assert()
        .success()
        .stdout("ab\n");

    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .write_stdin("ab")
        .assert()
        .success()
        .stdout("ab");

    Ok(())
}

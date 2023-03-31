mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn loop_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_x)\"\"=(_n)<1>=/(_n)(_x)*(_o)o.?/(_x)(_n)<0>=(_x)A!\\(_x)\"a\"=(_n)*(_o)(on).?\\")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1a0");

    Ok(())
}

#[test]
fn loop_unset_local() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("/(_z)\\")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn loop_unset_member() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("/z\\")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn loop_unset_global() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("/(GLOBAL)\\")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

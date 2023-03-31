mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn comma_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1><2>,(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1");

    Ok(())
}

#[test]
fn dollar_sign_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(",")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

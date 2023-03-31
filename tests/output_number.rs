mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn output_number_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<12.5e-1>(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1.25");

    Ok(())
}

#[test]
fn output_number_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(name)(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn output_number_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

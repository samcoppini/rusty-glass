mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn math_less_than_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<11><24>(_a)(lt).?(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1");

    let file = glass_expression_file("<11><11>(_a)(lt).?(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("0");

    let file = glass_expression_file("<200><24>(_a)(lt).?(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("0");

    Ok(())
}

#[test]
fn math_less_than_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<11>\"12\"(_a)(lt).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("(name)<11>(_a)(lt).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn math_less_than_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<11>(_a)(lt).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("(_a)(lt).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

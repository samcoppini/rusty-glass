mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn math_modulo_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<24><11>(_a)(mod).?(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("2");

    Ok(())
}

#[test]
fn math_modulo_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<11>\"12\"(_a)(mod).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("(name)<11>(_a)(mod).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn math_modulo_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<11>(_a)(mod).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("(_a)(mod).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

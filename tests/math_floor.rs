mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn math_floor_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<12.1>(_a)f.?(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("12");

    let file = glass_expression_file("<-12.1>(_a)f.?(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("-13");

    let file = glass_expression_file("<100>(_a)f.?(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .success()
        .stdout("100");

    Ok(())
}

#[test]
fn math_floor_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("\"12\"(_a)f.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn math_floor_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_a)f.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn output_string_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("\"Hey\"(_o)o.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("Hey");

    Ok(())
}

#[test]
fn output_string_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>(_o)o.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn output_string_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_o)o.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

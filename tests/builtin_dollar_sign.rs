mod common;

use crate::common::glass_file;
use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn local_dollar_sign() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[m(_t)$(_t)a.?][a(_o)O!<1>(_o)(on).?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1");

    Ok(())
}

#[test]
fn member_dollar_sign() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[mt$(_t)$(_t)a.?][atx.?][x(_o)O!<2>(_o)(on).?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("2");

    Ok(())
}

#[test]
fn global_dollar_sign() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[mX$(_n)N!(_n)a.?][a(_o)O!<3>(_o)(on).?]}{N[aXa.?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("3");

    Ok(())
}

#[test]
fn dollar_sign_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>$")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn dollar_sign_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("$")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

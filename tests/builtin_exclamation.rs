mod common;

use crate::common::glass_file;
use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn local_exclamation() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[m(_t)M!(_t)a.?][a(_o)O!<1>(_o)(on).?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1");

    Ok(())
}

#[test]
fn member_exclamation() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[mxM!(_t)$(_t)a.?][axy.?][y(_o)O!<2>(_o)(on).?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("2");

    Ok(())
}

#[test]
fn global_exclamation() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[mZX!aY!ay.?]}{Y[yZy.?]}{X[y(_o)O!<3>(_o)(on).?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("3");

    Ok(())
}

#[test]
fn exclamation_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>A!")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("(_name)\"A\"!")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn exclamation_not_a_class() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_a)X!")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("A\"not a class\"=(_a)A!")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn exclamation_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("A!")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("!")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

mod common;

use crate::common::glass_file;
use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn local_equal() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[m(_t)<1>=(_t)*(_o)O!(_o)(on).?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1");

    Ok(())
}

#[test]
fn member_equal() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[ma\"2\"=(_t)$(_t)x.?][x(_o)O!a*(_o)o.?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("2");

    Ok(())
}

#[test]
fn global_equal() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[mA\"3\"=(_t)T!(_t)o.?]}{T[o(_o)O!A*(_o)o.?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("3");

    Ok(())
}

#[test]
fn equal_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<2><3>=")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn equal_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<3>=")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("=")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

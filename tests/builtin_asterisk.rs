mod common;

use crate::common::glass_file;
use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn local_asterisk() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[m(_t)<1>=(_t)*(_o)O!(_o)(on).?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1");

    Ok(())
}

#[test]
fn member_asterisk() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[ma\"2\"=(_t)$(_t)x.?][x(_o)O!a*(_o)o.?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("2");

    Ok(())
}

#[test]
fn global_asterisk() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[mA\"3\"=(_t)T!(_t)o.?]}{T[o(_o)O!A*(_o)o.?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("3");

    Ok(())
}

#[test]
fn asterisk_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>*")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn asterisk_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("*")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn asterisk_unset_global() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("X*")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn asterisk_unset_member() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[ma<1>=zM!zz.?][za*]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn asterisk_unset_local() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"{M[m/A(_a)*\\(_a)<1>=A<1>=(_t)$(_t)m.?]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

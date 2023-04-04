mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn string_index_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        "blah" <3> (_s)i.? (_o)o.?
        "blah" <2> (_s)i.? (_o)o.?
        "blah" <1> (_s)i.? (_o)o.?
        "blah" <0> (_s)i.? (_o)o.?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("halb");

    Ok(())
}

#[test]
fn string_index_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1><1>(_s)i.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file(r#""hello""1"(_s)i.?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn string_index_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>(_s)i.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file(r"(_s)i.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn string_index_bad_indices() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#""blah"<1.1>(_s)i.?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file(r#""blah"<5>(_s)i.?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file(r#""blah"<-1>(_s)i.?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

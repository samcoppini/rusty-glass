mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn string_concat() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        (_str)""=
        (_n)<5>=
        /(_n)
            (_str)*(_o)o.?
            "\n"(_o)o.?
            (_str)(_str)*"!"(_s)a.?=
            (_n)(_n)*<1>(_a)s.?=
        \
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("\n!\n!!\n!!!\n!!!!\n");

    Ok(())
}

#[test]
fn concat_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>\"a\"(_s)a.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("\"a\"<1>(_s)a.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn concat_empty_stack()-> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("\"a\"(_s)a.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("(_s)a.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

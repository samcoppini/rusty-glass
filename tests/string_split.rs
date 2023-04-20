mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn string_split_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        (_i)<0>=
        (_lt)<1>=
        /(_lt)
            "abcde"(_i)*(_s)d.?
            1(_o)o.?" "(_o)o.?(_o)o.?"\n"(_o)o.?
            (_i)(_i)*<1>(_a)a.?=
            (_lt)(_i)*<6>(_a)(lt).?=
        \
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout(" abcde\na bcde\nab cde\nabc de\nabcd e\nabcde \n");

    Ok(())
}

#[test]
fn string_split_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("\"abc\"<10>(_s)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file("\"abc\"<1.5>(_s)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file("\"abc\"<-1>(_s)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file("\"abc\"\"1\"(_s)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file("<12><1>(_s)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn string_split_empty_stack()-> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<10>(_s)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file("(_s)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

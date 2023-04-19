mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn string_replace_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        "aaaaa"<3>"b"(_s)(si).?(_o)o.?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("aaaba");

    Ok(())
}

#[test]
fn string_replace_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#""aaaa"<1><2>(_s)(si).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file(r#""aaaa""x""b"(_s)(si).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file(r#"(_notastring)<1>"b"(_s)(si).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file(r#""aaaa"<1>"long replacement"(_s)(si).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn string_replace_empty_stack()-> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"<1>"a"(_s)(si).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file(r#""a"(_s)(si).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file(r#"(_s)(si).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

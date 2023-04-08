mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn num_to_string_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        <65> (_s)(ns).? (_o)o.?
        <97> (_s)(ns).? (_o)o.?
        <32> (_s)(ns).? (_o)o.?
        <48> (_s)(ns).? (_o)o.?
        <10> (_s)(ns).? (_o)o.?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("Aa 0\n");

    Ok(())
}

#[test]
fn num_to_string_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("\"a\"(_s)(ns).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("<1.1>(_s)(ns).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file("<1000>(_s)(ns).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    let file = glass_expression_file("<-100>(_s)(ns).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn num_to_string_empty_stack()-> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_s)(ns).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

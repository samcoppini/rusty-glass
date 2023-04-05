mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn string_to_num_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        " " (_s)(sn).? (_o)(on).? "\n" (_o)o.?
        "0" (_s)(sn).? (_o)(on).? "\n" (_o)o.?
        "a" (_s)(sn).? (_o)(on).? "\n" (_o)o.?
        "A" (_s)(sn).? (_o)(on).? "\n" (_o)o.?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("32\n48\n97\n65\n");

    Ok(())
}

#[test]
fn string_to_num_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>(_s)(sn).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file(r#""hello"(_s)(sn).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file(r#"""(_s)(sn).?"#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn string_to_num_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_s)(sn).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

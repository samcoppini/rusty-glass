mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn string_length_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        ""  (_s)l.? (_o)(on).?
        "whattheheck" (_s)l.? (_o)(on).?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("011");

    Ok(())
}

#[test]
fn string_length_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>(_s)l.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn string_length_empty_stack()-> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_s)l.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

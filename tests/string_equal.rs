mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn string_equal_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        "abc"  "abc"  (_s)e.? (_o)(on).?
        "abc"  "abcd" (_s)e.? (_o)(on).?
        "abcd" "abc"  (_s)e.? (_o)(on).?
        "abd"  "abc"  (_s)e.? (_o)(on).?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1000");

    Ok(())
}

#[test]
fn string_equal_wrong_types() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>\"a\"(_s)e.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("\"a\"<1>(_s)e.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

#[test]
fn string_equal_empty_stack()-> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("\"a\"(_s)e.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    let file = glass_expression_file("(_s)e.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
        .assert()
        .failure();

    Ok(())
}

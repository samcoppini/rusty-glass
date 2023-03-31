mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn duplicate() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("\"2\"\"1\"\"0\"2(_o)o.?1(_o)o.?0(_o)o.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("210");

    Ok(())
}

#[test]
fn duplicate_out_of_range() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1><2><3><4><5><6><7><8>9")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

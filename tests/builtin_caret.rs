mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn caret_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("<1>(_o)(on).?^<2>(_o)(on).?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("1");

    Ok(())
}

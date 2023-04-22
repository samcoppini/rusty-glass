mod common;

use crate::common::glass_file;
use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn auto_vars_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&r#"
        {M[moO!vV!vn.?000000Z!z.?*oo.?vn.?X!*oo.?vd.?]}
        {Z[(c__)oO!"a"oo.?][z"b"oo.?x.?][x"c"oo.?"d"=]}
        {X[(c__)oO!"e"oo.?]}
    "#.to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("abcded");

    Ok(())
}

#[test]
fn auto_var_delete_wrong_type() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_x)(_v)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn auto_var_delete_empty_stack() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_v)d.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

#[test]
fn auto_var_unset_var() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_v)n.?*")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .failure();

    Ok(())
}

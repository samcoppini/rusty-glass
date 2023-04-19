mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn input_char_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file("(_i)c.0?1?2?3?4?(_o)o.?(_o)o.?(_o)o.?(_o)o.?(_o)o.?")?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .write_stdin("a\nb")
       .assert()
       .success()
       .stdout("\0\0b\na");

    Ok(())
}

mod common;

use crate::common::glass_expression_file;

use assert_cmd::Command;

#[test]
fn input_eof_success() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_expression_file(r#"
        (_i)e.?(_o)(on).?(_i)c.?(_o)o.?
        (_i)e.?(_o)(on).?(_i)c.?(_o)o.?
        (_i)e.?(_o)(on).?(_i)c.?(_o)o.?
        (_i)e.?(_o)(on).?(_i)c.?(_o)o.?
        (_i)e.?(_o)(on).?(_i)c.?(_o)o.?
    "#)?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .write_stdin("ab")
       .assert()
       .success()
       .stdout("0a0b0\01\01\0");

    Ok(())
}

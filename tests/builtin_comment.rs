mod common;

use crate::common::glass_file;

use assert_cmd::Command;

#[test]
fn comment_test() -> Result<(), Box<dyn std::error::Error>> {
    let file = glass_file(&"'comment'{'comment'M'comment'''['comment'm(_o)'c'O!<8>(_o)'X'(on).'48'?/'X'(_o)''\\]}".to_owned())?;
    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(file.path())
       .assert()
       .success()
       .stdout("8");

    Ok(())
}

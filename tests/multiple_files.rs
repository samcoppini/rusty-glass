mod common;

use crate::common::glass_file;

use assert_cmd::Command;

#[test]
pub fn multiple_files_test() -> Result<(), Box<dyn std::error::Error>> {
    let main_file = glass_file(&"{M[m(_c2)(C2)!]}".to_owned())?;
    let file2 = glass_file(&"{(C2)[(c__)(_c3)(C3)!]}".to_owned())?;
    let file3 = glass_file(&"{(C3)[(c__)(_c4)(C4)!]}".to_owned())?;
    let file4 = glass_file(&"{(C4)[(c__)(_c5)(C5)!]}".to_owned())?;
    let file5 = glass_file(&"{(C5)[(c__)(_o)O!<42>(_o)(on).?]}".to_owned())?;

    let mut cmd = Command::cargo_bin("glass")?;

    cmd.arg(main_file.path())
       .arg(file2.path())
       .arg(file3.path())
       .arg(file4.path())
       .arg(file5.path())
       .assert()
       .success()
       .stdout("42");

    Ok(())
}

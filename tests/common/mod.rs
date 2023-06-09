use assert_fs::NamedTempFile;
use assert_fs::fixture::FileWriteStr;

#[allow(dead_code)]
pub fn glass_expression_file(code: &str) -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    glass_file(&format!("{{M[m(_a)A!(_i)I!(_o)O!(_s)S!(_v)V!{}]}}", code).to_owned())
}

pub fn glass_file(code: &String) -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("glass.txt")?;
    file.write_str(code)?;
    Ok(file)
}

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    let _ = file.read_to_string(&mut content)?;

    Ok(content)
}

use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

pub fn read_file(path: &Path) -> io::Result<String> {
    let mut c_code = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut c_code)?;
    Ok(c_code)
}

pub fn delete_file(path: &Path) -> io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

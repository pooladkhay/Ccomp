use std::{
    io::{self, Write},
    path::Path,
    process::{Command, ExitStatus},
};

/// Runs `gcc -E -P INPUT_FILE -o PREPROCESSED_FILE`, prints it's outputs and returns it's status code.
pub fn preprocessor(file_path: &Path) -> Result<(), ExitStatus> {
    let preprocessor = Command::new("gcc")
        .args([
            "-E",
            "-P",
            file_path.to_str().unwrap(),
            "-o",
            &format!("{}.i", file_path.file_stem().unwrap().to_str().unwrap()),
        ])
        .output()
        .expect("failed to run preprocessor");

    io::stdout().write_all(&preprocessor.stdout).unwrap();
    io::stderr().write_all(&preprocessor.stderr).unwrap();

    if !preprocessor.status.success() {
        return Err(preprocessor.status);
    }

    Ok(())
}

use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process::{exit, Command},
};

/// Runs `gcc -E -P INPUT_FILE -o PREPROCESSED_FILE`, and returns a `PathBuf` to the preprocessed file.
///
/// # Exits
///
/// This function will terminate the process with a status code of `1` if the preprocess command fails.
pub fn preprocessor(file_path: &Path) -> PathBuf {
    let preprocessed_file_path = format!("{}.i", file_path.file_stem().unwrap().to_str().unwrap());

    let preprocessor = Command::new("gcc")
        .args([
            "-E",
            "-P",
            file_path.to_str().unwrap(),
            "-o",
            &preprocessed_file_path,
        ])
        .output()
        .expect("failed to run preprocessor");

    io::stdout().write_all(&preprocessor.stdout).unwrap();
    io::stderr().write_all(&preprocessor.stderr).unwrap();

    if !preprocessor.status.success() {
        eprintln!("preprocessor failed.");
        exit(preprocessor.status.code().unwrap())
    }

    PathBuf::from(preprocessed_file_path)
}

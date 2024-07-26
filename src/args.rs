use std::path::PathBuf;

use crate::CompileStage;

pub fn parse(args: Vec<String>) -> (PathBuf, CompileStage) {
    let mut flags = 0_u8;
    let mut file_path = PathBuf::new();

    args.into_iter().enumerate().for_each(|(i, arg)| {
        if i == 0 {
            return;
        }
        match arg.as_ref() {
            "--lex" => flags |= 0x01,
            "--parse" => flags |= 0x02,
            "--code-gen" => flags |= 0x04,
            "-S" => flags |= 0x08,
            "--help" | "-h" => panic!("Help is not implemented yet."),
            arg if arg.ends_with(".c") => file_path.push(arg),
            arg => panic!("Unknown argument: '{arg}'. Use -h or --help flag for more information."),
        }
    });

    if file_path.cmp(&PathBuf::new()).is_eq() {
        panic!("No source file is specified. Use -h or --help flag for more information.")
    }

    match flags {
        0 => (file_path, CompileStage::All),
        1 => (file_path, CompileStage::Lex),
        2 => (file_path, CompileStage::Parse),
        4 => (file_path, CompileStage::CodeGen),
        8 => (file_path, CompileStage::EmitCode),
        _ => panic!("Only one of the '--lex', '--parse', '--code-gen', or '-S' flags should be passed. Use -h or --help flag for more information."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::OsStr, path::Path};

    #[test]
    fn test_parse_path() {
        let args = vec!["program".to_string(), "/path/to/file.c".to_string()];
        let (file_path, compile_stage) = parse(args);

        assert_eq!(file_path.file_name(), Some(OsStr::new("file.c")));
        assert_eq!(compile_stage, CompileStage::All);
    }

    #[test]
    fn test_parse_no_flag() {
        let args = vec!["program".to_string(), "file.c".to_string()];
        let result = parse(args);
        assert_eq!(result, (Path::new("file.c").to_owned(), CompileStage::All));
    }

    #[test]
    fn test_parse_lex_flag() {
        let args = vec![
            "program".to_string(),
            "--lex".to_string(),
            "file.c".to_string(),
        ];
        let result = parse(args);
        assert_eq!(result, (Path::new("file.c").to_owned(), CompileStage::Lex));
    }

    #[test]
    fn test_parse_parse_flag() {
        let args = vec![
            "program".to_string(),
            "--parse".to_string(),
            "file.c".to_string(),
        ];
        let result = parse(args);
        assert_eq!(
            result,
            (Path::new("file.c").to_owned(), CompileStage::Parse)
        );
    }

    #[test]
    fn test_parse_code_gen_flag() {
        let args = vec![
            "program".to_string(),
            "--code-gen".to_string(),
            "file.c".to_string(),
        ];
        let result = parse(args);
        assert_eq!(
            result,
            (Path::new("file.c").to_owned(), CompileStage::CodeGen)
        );
    }

    #[test]
    fn test_parse_emit_assembly_flag() {
        let args = vec![
            "program".to_string(),
            "-S".to_string(),
            "file.c".to_string(),
        ];
        let result = parse(args);
        assert_eq!(
            result,
            (Path::new("file.c").to_owned(), CompileStage::EmitCode)
        );
    }

    #[test]
    #[should_panic(expected = "No source file is specified.")]
    fn test_parse_no_file() {
        let args = vec!["program".to_string()];
        parse(args);
    }

    #[test]
    #[should_panic(expected = "No source file is specified.")]
    fn test_parse_no_file_with_flag() {
        let args = vec!["program".to_string(), "--lex".to_string()];
        parse(args);
    }

    #[test]
    #[should_panic(expected = "Unknown argument: '--unknown'.")]
    fn test_parse_unknown_flag() {
        let args = vec![
            "program".to_string(),
            "file.c".to_string(),
            "--unknown".to_string(),
        ];
        parse(args);
    }

    #[test]
    #[should_panic(
        expected = "Only one of the '--lex', '--parse', '--code-gen', or '-S' flags should be passed."
    )]
    fn test_parse_multiple_flags() {
        let args = vec![
            "program".to_string(),
            "--lex".to_string(),
            "--parse".to_string(),
            "file.c".to_string(),
        ];
        parse(args);
    }
}

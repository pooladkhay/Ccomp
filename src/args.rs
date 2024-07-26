use crate::CompileStage;

pub fn parse(args: Vec<String>) -> (String, CompileStage) {
    let mut flags = 0_u8;
    let mut file_path = String::new();

    args.into_iter().enumerate().for_each(|(i, arg)| {
        if i == 0 {
            return;
        }
        match arg.as_ref() {
            "--lex" => flags |= 0x01,
            "--parse" => flags |= 0x02,
            "--code-gen" => flags |= 0x04,
            "--help" | "-h" => panic!("Help is not implemented yet."),
            arg if arg.ends_with(".c") => file_path = arg.to_string(),
            arg => panic!("Unknown argument: '{arg}'. Use -h or --help flag for more information."),
        }
    });

    if file_path.is_empty() {
        panic!("No source file is specified. Use -h or --help flag for more information.")
    }

    match flags {
        0 => (file_path, CompileStage::All),
        1 => (file_path, CompileStage::Lex),
        2 => (file_path, CompileStage::Parse),
        4 => (file_path, CompileStage::CodeGen),
        _ => panic!("Only one of the '--lex', '--parse', or '--code-gen' flags should be passed. Use -h or --help flag for more information."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_flag() {
        let args = vec!["program".to_string(), "file.c".to_string()];
        let result = parse(args);
        assert_eq!(result, ("file.c".to_string(), CompileStage::All));
    }

    #[test]
    fn test_parse_lex_flag() {
        let args = vec![
            "program".to_string(),
            "--lex".to_string(),
            "file.c".to_string(),
        ];
        let result = parse(args);
        assert_eq!(result, ("file.c".to_string(), CompileStage::Lex));
    }

    #[test]
    fn test_parse_parse_flag() {
        let args = vec![
            "program".to_string(),
            "--parse".to_string(),
            "file.c".to_string(),
        ];
        let result = parse(args);
        assert_eq!(result, ("file.c".to_string(), CompileStage::Parse));
    }

    #[test]
    fn test_parse_code_gen_flag() {
        let args = vec![
            "program".to_string(),
            "--code-gen".to_string(),
            "file.c".to_string(),
        ];
        let result = parse(args);
        assert_eq!(result, ("file.c".to_string(), CompileStage::CodeGen));
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
        expected = "Only one of the '--lex', '--parse', or '--code-gen' flags should be passed."
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

use std::{env, process};

use lexer::Lexer;

mod args;
mod cc;
mod helper;
mod lexer;

#[derive(Debug, PartialEq)]
enum CompileStage {
    Lex,
    Parse,
    CodeGen,
    EmitCode,
    All,
}

fn main() {
    let (c_file_path, compile_stage) = args::parse(env::args().collect::<Vec<String>>());

    //-------------------------
    // Preprocessor
    let preprocessed_c_file_path = cc::preprocessor(c_file_path.as_path());

    //-------------------------
    // Compiler

    // Lexer
    let preprocessed_c_code = helper::read_file(preprocessed_c_file_path.as_path()).unwrap();
    helper::delete_file(preprocessed_c_file_path.as_path()).unwrap();

    let mut lexer = Lexer::new(preprocessed_c_code);
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }

    // Exit if '--lex' flag was passed
    match compile_stage {
        CompileStage::Lex => process::exit(0),
        _ => (),
    }

    // Parser
    // ...

    // Assembly Generation
    // ...

    // Code Emission
    // ...

    //-------------------------
    // Assembler and Linker
    // gcc ASSEMBLY_FILE -o OUTPUT_FILE
}

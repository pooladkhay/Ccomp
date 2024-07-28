use std::env;

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

    println!("compile stage: {:?}", compile_stage);
    println!("c file path: {:?}", c_file_path);

    //-------------------------
    // Preprocessor
    let preprocessed_c_file_path = cc::preprocessor(c_file_path.as_path());

    //-------------------------
    // Compiler

    // Lexer
    let preprocessed_c_code = helper::read_file(preprocessed_c_file_path.as_path()).unwrap();
    helper::delete_file(preprocessed_c_file_path.as_path()).unwrap();

    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(preprocessed_c_code);
    for token in tokens {
        println!("{:?}", token);
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

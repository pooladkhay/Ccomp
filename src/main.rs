use std::{env, process::exit};

mod args;
mod cc;

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

    // ---- PREPROCESSOR
    if let Err(status) = cc::preprocessor(c_file_path.as_path()) {
        eprintln!("preprocessor failed.");
        exit(status.code().unwrap())
    }

    // ---- COMPILE
    //	Lexer
    //	Parser
    //	Assembly Generation
    //	Code Emission

    // ---- ASSEMBLE and LINK
    //	gcc ASSEMBLY_FILE -o OUTPUT_FILE
}

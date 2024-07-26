use std::env;

mod args;

#[derive(Debug, PartialEq)]
enum CompileStage {
    Lex,
    Parse,
    CodeGen,
    All,
}

fn main() {
    let (c_file_path, compile_stage) = args::parse(env::args().collect::<Vec<String>>());

    println!("{:?}", compile_stage);
    println!("{:?}", c_file_path);

    // ---- PREPROCESS
    //	gcc -E -P INPUT_FILE -o PREPROCESSED_FILE

    // ---- COMPILE
    //	Lexer
    //	Parser
    //	Assembly Generation
    //	Code Emission

    // ---- ASSEMBLE and LINK
    //	gcc ASSEMBLY_FILE -o OUTPUT_FILE
}

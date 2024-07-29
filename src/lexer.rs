use regex::Regex;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Keyword {
    Return,
    Int,
    Void,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Identifier {
        line: usize,
        column: usize,
        value: String,
    },
    Keyword {
        line: usize,
        column: usize,
        value: Keyword,
    },
    Constant {
        line: usize,
        column: usize,
        value: String,
    },
    OpenParen {
        line: usize,
        column: usize,
    },
    CloseParen {
        line: usize,
        column: usize,
    },
    OpenBrace {
        line: usize,
        column: usize,
    },
    CloseBrace {
        line: usize,
        column: usize,
    },
    Semicolon {
        line: usize,
        column: usize,
    },
}

pub struct Lexer {
    code: String,

    identifier_re: Regex,
    constant_re: Regex,

    // non-whitespace boundaries
    boundary: Vec<char>,

    line: usize,
    column: usize,

    start: usize,
    end: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            code,

            identifier_re: Regex::new(r"^[a-zA-Z_]\w*$").unwrap(),
            constant_re: Regex::new(r"^[0-9]+$").unwrap(),

            boundary: vec![';', '(', ')', '{', '}'],

            line: 1,
            column: 1,

            start: 0,
            end: 0,
        }
    }

    /// Converts a string containing "C code" to `Vec<Token>`.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut iter_peek = self.code.chars().peekable();

        loop {
            match iter_peek.next() {
                Some(ch) => {
                    if ch.is_whitespace() {
                        self.end += 1;
                        self.column += self.end - self.start;
                        self.start = self.end;

                        if ch == '\n' {
                            self.line += 1;
                            self.column = 1;
                        }

                        continue;
                    }

                    self.end += 1;

                    match iter_peek.peek() {
                        Some(next) => {
                            if next.is_whitespace()
                                || self.boundary.contains(next)
                                || self.boundary.contains(&ch)
                            {
                                if self.start != self.end {
                                    tokens.push(self.gen_token());
                                }
                                if self.boundary.contains(next) || self.boundary.contains(&ch) {
                                    self.column += self.end - self.start;
                                    self.start = self.end;
                                }
                            }
                        }
                        None => {
                            if self.start != self.end {
                                tokens.push(self.gen_token());
                            }
                        }
                    }
                }
                None => break,
            }
        }

        tokens
    }

    fn gen_token(&self) -> Token {
        match &self.code[self.start..self.end] {
            ";" => Token::Semicolon {
                line: self.line,
                column: self.column,
            },
            "(" => Token::OpenParen {
                line: self.line,
                column: self.column,
            },
            ")" => Token::CloseParen {
                line: self.line,
                column: self.column,
            },
            "{" => Token::OpenBrace {
                line: self.line,
                column: self.column,
            },
            "}" => Token::CloseBrace {
                line: self.line,
                column: self.column,
            },
            "int" => Token::Keyword {
                line: self.line,
                column: self.column,
                value: Keyword::Int,
            },
            "void" => Token::Keyword {
                line: self.line,
                column: self.column,
                value: Keyword::Void,
            },
            "return" => Token::Keyword {
                line: self.line,
                column: self.column,
                value: Keyword::Return,
            },
            t => {
                if self.identifier_re.is_match(t) {
                    Token::Identifier {
                        line: self.line,
                        column: self.column,
                        value: t.to_owned(),
                    }
                } else if self.constant_re.is_match(t) {
                    Token::Constant {
                        line: self.line,
                        column: self.column,
                        value: t.to_owned(),
                    }
                } else {
                    panic!(
                        "unknown token '{}' at line {}, column {}.",
                        t, self.line, self.column
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "unknown token '@void' at line 1, column 5.")]
    fn test_tokenize_unknown_token() {
        let mut lexer = Lexer::new(String::from("int @void return"));
        let _tokens = lexer.tokenize();
    }

    #[test]
    fn test_tokenize_keywords() {
        let mut lexer = Lexer::new(String::from("int void return"));
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 3);

        match &tokens[0] {
            Token::Keyword {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 1);
                assert_eq!(*value, Keyword::Int);
            }
            _ => panic!("Expected a Keyword token"),
        }

        match &tokens[1] {
            Token::Keyword {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 5);
                assert_eq!(*value, Keyword::Void);
            }
            _ => panic!("Expected a Keyword token"),
        }

        match &tokens[2] {
            Token::Keyword {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 10);
                assert_eq!(*value, Keyword::Return);
            }
            _ => panic!("Expected a Keyword token"),
        }
    }

    #[test]
    fn test_tokenize_identifiers() {
        let mut lexer = Lexer::new(String::from("var1 _var2 VAR_3"));
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 3);

        match &tokens[0] {
            Token::Identifier {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 1);
                assert_eq!(value, "var1");
            }
            _ => panic!("Expected an Identifier token"),
        }

        match &tokens[1] {
            Token::Identifier {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 6);
                assert_eq!(value, "_var2");
            }
            _ => panic!("Expected an Identifier token"),
        }

        match &tokens[2] {
            Token::Identifier {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 12);
                assert_eq!(value, "VAR_3");
            }
            _ => panic!("Expected an Identifier token"),
        }
    }

    #[test]
    fn test_tokenize_constants() {
        let mut lexer = Lexer::new(String::from("123 456 789"));
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 3);

        match &tokens[0] {
            Token::Constant {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 1);
                assert_eq!(value, "123");
            }
            _ => panic!("Expected a Constant token"),
        }

        match &tokens[1] {
            Token::Constant {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 5);
                assert_eq!(value, "456");
            }
            _ => panic!("Expected a Constant token"),
        }

        match &tokens[2] {
            Token::Constant {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 9);
                assert_eq!(value, "789");
            }
            _ => panic!("Expected a Constant token"),
        }
    }

    #[test]
    fn test_tokenize_delimiters() {
        let mut lexer = Lexer::new(String::from("(){};"));
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5);

        match &tokens[0] {
            Token::OpenParen { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 1);
            }
            _ => panic!("Expected an OpenParen token"),
        }

        match &tokens[1] {
            Token::CloseParen { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 2);
            }
            _ => panic!("Expected a CloseParen token"),
        }

        match &tokens[2] {
            Token::OpenBrace { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 3);
            }
            _ => panic!("Expected an OpenBrace token"),
        }

        match &tokens[3] {
            Token::CloseBrace { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 4);
            }
            _ => panic!("Expected a CloseBrace token"),
        }

        match &tokens[4] {
            Token::Semicolon { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 5);
            }
            _ => panic!("Expected a Semicolon token"),
        }
    }

    #[test]
    fn test_tokenize_mixed() {
        let mut lexer = Lexer::new(String::from("int main() { return 42; }"));
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 9);

        match &tokens[0] {
            Token::Keyword {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 1);
                assert_eq!(*value, Keyword::Int);
            }
            _ => panic!("Expected a Keyword token"),
        }

        match &tokens[1] {
            Token::Identifier {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 5);
                assert_eq!(value, "main");
            }
            _ => panic!("Expected an Identifier token"),
        }

        match &tokens[2] {
            Token::OpenParen { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 9);
            }
            _ => panic!("Expected an OpenParen token"),
        }

        match &tokens[3] {
            Token::CloseParen { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 10);
            }
            _ => panic!("Expected a CloseParen token"),
        }

        match &tokens[4] {
            Token::OpenBrace { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 12);
            }
            _ => panic!("Expected an OpenBrace token"),
        }

        match &tokens[5] {
            Token::Keyword {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 14);
                assert_eq!(*value, Keyword::Return);
            }
            _ => panic!("Expected a Keyword token"),
        }

        match &tokens[6] {
            Token::Constant {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 21);
                assert_eq!(value, "42");
            }
            _ => panic!("Expected a Constant token"),
        }

        match &tokens[7] {
            Token::Semicolon { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 23);
            }
            _ => panic!("Expected a Semicolon token"),
        }

        match &tokens[8] {
            Token::CloseBrace { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 25);
            }
            _ => panic!("Expected a CloseBrace token"),
        }
    }

    #[test]
    fn test_tokenize_mixed_with_newline() {
        let mut lexer = Lexer::new(String::from("int   main() \n  {    \n return 42; \n}"));
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 9);

        match &tokens[0] {
            Token::Keyword {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 1);
                assert_eq!(*value, Keyword::Int);
            }
            _ => panic!("Expected a Keyword token"),
        }

        match &tokens[1] {
            Token::Identifier {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 7);
                assert_eq!(value, "main");
            }
            _ => panic!("Expected an Identifier token"),
        }

        match &tokens[2] {
            Token::OpenParen { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 11);
            }
            _ => panic!("Expected an OpenParen token"),
        }

        match &tokens[3] {
            Token::CloseParen { line, column } => {
                assert_eq!(*line, 1);
                assert_eq!(*column, 12);
            }
            _ => panic!("Expected a CloseParen token"),
        }

        match &tokens[4] {
            Token::OpenBrace { line, column } => {
                assert_eq!(*line, 2);
                assert_eq!(*column, 3);
            }
            _ => panic!("Expected an OpenBrace token"),
        }

        match &tokens[5] {
            Token::Keyword {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 3);
                assert_eq!(*column, 2);
                assert_eq!(*value, Keyword::Return);
            }
            _ => panic!("Expected a Keyword token"),
        }

        match &tokens[6] {
            Token::Constant {
                line,
                column,
                value,
            } => {
                assert_eq!(*line, 3);
                assert_eq!(*column, 9);
                assert_eq!(value, "42");
            }
            _ => panic!("Expected a Constant token"),
        }

        match &tokens[7] {
            Token::Semicolon { line, column } => {
                assert_eq!(*line, 3);
                assert_eq!(*column, 11);
            }
            _ => panic!("Expected a Semicolon token"),
        }

        match &tokens[8] {
            Token::CloseBrace { line, column } => {
                assert_eq!(*line, 4);
                assert_eq!(*column, 1);
            }
            _ => panic!("Expected a CloseBrace token"),
        }
    }
}

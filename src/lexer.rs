use std::process::exit;

use regex::Regex;

#[allow(dead_code)]
#[derive(Debug)]
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
    pub fn new() -> Self {
        Self {
            identifier_re: Regex::new(r"[a-zA-Z_]\w*\b").unwrap(),
            constant_re: Regex::new(r"[0-9]+\b").unwrap(),

            boundary: vec![';', '(', ')', '{', '}'],

            line: 1,
            column: 0,

            start: 0,
            end: 0,
        }
    }

    /// Converts a string containing "C code" to `Vec<Token>`.
    pub fn tokenize(&mut self, code: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        for ch in code.chars() {
            if ch.is_whitespace() || self.boundary.contains(&ch) {
                self.column += 1;

                if ch == '\n' {
                    self.line += 1;
                    self.column = 0;
                }

                if self.start != self.end {
                    match &code[self.start..self.end] {
                        "int" => tokens.push(Token::Keyword {
                            line: self.line,
                            column: self.column,
                            value: Keyword::Int,
                        }),
                        "void" => tokens.push(Token::Keyword {
                            line: self.line,
                            column: self.column,
                            value: Keyword::Void,
                        }),
                        "return" => tokens.push(Token::Keyword {
                            line: self.line,
                            column: self.column,
                            value: Keyword::Return,
                        }),
                        t => {
                            if self.identifier_re.is_match(t) {
                                tokens.push(Token::Identifier {
                                    line: self.line,
                                    column: self.column,
                                    value: t.to_owned(),
                                })
                            } else if self.constant_re.is_match(t) {
                                tokens.push(Token::Constant {
                                    line: self.line,
                                    column: self.column,
                                    value: t.to_owned(),
                                })
                            } else {
                                eprintln!(
                                    "unknown token '{}' at line {}, column {}.",
                                    t, self.line, self.column
                                );
                                exit(1)
                            }
                        }
                    }
                    self.column += self.end - self.start;
                }

                if self.boundary.contains(&ch) {
                    match ch {
                        ';' => tokens.push(Token::Semicolon {
                            line: self.line,
                            column: self.column,
                        }),
                        '(' => tokens.push(Token::OpenParen {
                            line: self.line,
                            column: self.column,
                        }),
                        ')' => tokens.push(Token::CloseParen {
                            line: self.line,
                            column: self.column,
                        }),
                        '{' => tokens.push(Token::OpenBrace {
                            line: self.line,
                            column: self.column,
                        }),
                        '}' => tokens.push(Token::CloseBrace {
                            line: self.line,
                            column: self.column,
                        }),
                        b => {
                            eprintln!(
                                "unknown token '{}' at line {}, column {}.",
                                b, self.line, self.column
                            );
                            exit(1)
                        }
                    }
                }

                self.end += 1;
                self.start = self.end;

                continue;
            }
            self.end += 1;
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

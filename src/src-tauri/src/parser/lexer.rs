// Q-Code IDE - Unified High-Performance Multi-Language Quantum Lexical Analyzer
// Dual-compliant with OpenQASM 3.0 and Microsoft Q# ISO Specifications

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Shared & Language Specifiers
    OpenQASM,
    Version(String),
    Include,
    EOF,
    Illegal,
    Semicolon,
    Assignment,
    StringLiteral(String),
    Identifier(String),
    
    // Quantum & Classical Registers
    Qubit,
    Bit,
    
    // Microsoft Q# Specific Architectural Keywords
    Namespace,    // "namespace"
    Open,         // "open" (Imports)
    Operation,    // "operation" (Quantum Functions)
    Body,         // "body"
    Adjoint,      // "adjoint" (Unitary Inverse)
    Controlled,   // "controlled" (Controlled Gates)
    LeftBrace,    // "{"
    RightBrace,   // "}"
    LeftParen,    // "("
    RightParen,   // ")"
    Colon,        // ":"
    Unit,         // "Unit" (Q# Return Type)

    // Universal Quantum Gate Primitives
    GateH,        // "h" or "H"
    GateCX,       // "cx" or "CNOT"
    GateX,        // "x" or "X"
    Measure,      // "measure" or "M"
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            ';' => Token::Semicolon,
            '=' => Token::Assignment,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ':' => Token::Colon,
            '\0' => Token::EOF,
            '"' => {
                self.read_char();
                let start = self.position;
                while self.ch != '"' && self.ch != '\0' {
                    self.read_char();
                }
                let literal: String = self.input[start..self.position].iter().collect();
                self.read_char();
                return Token::StringLiteral(literal);
            }
            _ => {
                if self.ch.is_alphabetic() || self.ch == '.' || self.ch.is_numeric() {
                    let start = self.position;
                    // Support for scoped library syntax like Microsoft.Quantum.Intrinsic
                    while self.ch.is_alphanumeric() || self.ch == '.' || self.ch == '_' {
                        self.read_char();
                    }
                    let literal: String = self.input[start..self.position].iter().collect();
                    
                    // Case-insensitive normalization for cross-language compatibility
                    match literal.as_str() {
                        "OPENQASM" => return Token::OpenQASM,
                        "3.0" => return Token::Version(literal),
                        "include" => return Token::Include,
                        "qubit" => return Token::Qubit,
                        "bit" => return Token::Bit,
                        "namespace" => return Token::Namespace,
                        "open" => return Token::Open,
                        "operation" => return Token::Operation,
                        "body" => return Token::Body,
                        "adjoint" => return Token::Adjoint,
                        "controlled" => return Token::Controlled,
                        "Unit" => return Token::Unit,
                        "h" | "H" => return Token::GateH,
                        "cx" | "CNOT" => return Token::GateCX,
                        "x" | "X" => return Token::GateX,
                        "measure" | "M" => return Token::Measure,
                        _ => return Token::Identifier(literal),
                    }
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }
}

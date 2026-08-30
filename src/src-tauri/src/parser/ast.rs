// Q-Code IDE - Cross-Platform Quantum Abstract Syntax Tree (AST) Specifier
// Engineered to handle block-scoped structures and functional execution nodes

#[derive(Debug, Clone)]
pub enum ASTNode {
    // OpenQASM Specific Nodes
    Header { version: String },
    Include { file: String },
    
    // Microsoft Q# Specific Architectural Nodes
    NamespaceDeclaration { name: String },
    ModuleImport { library: String },
    OperationDeclaration { name: String, arguments: String, return_type: String },
    BlockScopeStart,
    BlockScopeEnd,

    // Universal Quantum Computational Nodes
    RegisterDeclaration { reg_type: String, name: String },
    QuantumGate { gate: String, target: String, is_controlled: bool, is_adjoint: bool },
    Measurement { source: String, target: String },
}

pub struct Parser {
    lexer: super::lexer::Lexer,
    current_token: super::lexer::Token,
    peek_token: super::lexer::Token,
}

impl Parser {
    pub fn new(mut lexer: super::lexer::Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser { lexer, current_token, peek_token }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut program = Vec::new();

        while self.current_token != super::lexer::Token::EOF {
            match &self.current_token {
                // Parse OpenQASM Headers
                super::lexer::Token::OpenQASM => {
                    self.next_token();
                    if let super::lexer::Token::Version(v) = &self.current_token {
                        program.push(ASTNode::Header { version: v.clone() });
                    }
                }
                super::lexer::Token::Include => {
                    self.next_token();
                    if let super::lexer::Token::StringLiteral(file) = &self.current_token {
                        program.push(ASTNode::Include { file: file.clone() });
                    }
                }
                
                // Parse Q# Namespace Architecture
                super::lexer::Token::Namespace => {
                    self.next_token();
                    if let super::lexer::Token::Identifier(name) = &self.current_token {
                        program.push(ASTNode::NamespaceDeclaration { name: name.clone() });
                    }
                }
                super::lexer::Token::Open => {
                    self.next_token();
                    if let super::lexer::Token::Identifier(lib) = &self.current_token {
                        program.push(ASTNode::ModuleImport { library: lib.clone() });
                    }
                }
                
                // Parse Q# Quantum Operations (Functions)
                super::lexer::Token::Operation => {
                    self.next_token();
                    if let super::lexer::Token::Identifier(op_name) = &self.current_token {
                        // Skip signatures and parameters for high-level syntax parsing layout
                        program.push(ASTNode::OperationDeclaration { 
                            name: op_name.clone(), 
                            arguments: "()".into(), 
                            return_type: "Unit".into() 
                        });
                    }
                }
                
                // Block Scope Mappings
                super::lexer::Token::LeftBrace => program.push(ASTNode::BlockScopeStart),
                super::lexer::Token::RightBrace => program.push(ASTNode::BlockScopeEnd),

                // Universal Gate Operations Mapping (Handles both dialects)
                super::lexer::Token::Qubit | super::lexer::Token::Bit => {
                    let reg_type = format!("{:?}", self.current_token);
                    self.next_token();
                    if let super::lexer::Token::Identifier(name) = &self.current_token {
                        program.push(ASTNode::RegisterDeclaration { reg_type, name: name.clone() });
                    }
                }
                super::lexer::Token::GateH => {
                    self.next_token();
                    // Handle dynamic brackets or namespaces if standard functional Q# style: H(q);
                    if self.current_token == super::lexer::Token::LeftParen { self.next_token(); }
                    if let super::lexer::Token::Identifier(target) = &self.current_token {
                        program.push(ASTNode::QuantumGate { 
                            gate: "H".into(), 
                            target: target.clone(),
                            is_controlled: false,
                            is_adjoint: false
                        });
                    }
                }
                super::lexer::Token::GateX => {
                    self.next_token();
                    if self.current_token == super::lexer::Token::LeftParen { self.next_token(); }
                    if let super::lexer::Token::Identifier(target) = &self.current_token {
                        program.push(ASTNode::QuantumGate { 
                            gate: "X".into(), 
                            target: target.clone(),
                            is_controlled: false,
                            is_adjoint: false
                        });
                    }
                }
                _ => {}
            }
            self.next_token();
        }

        Ok(program)
    }
}

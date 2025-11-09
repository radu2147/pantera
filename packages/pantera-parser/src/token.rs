use pantera_ast::expression::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: u128,
    pub typ: TokenType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Dot,
    Possesive,
    DoubleDot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Pow,
    Equal,
    Grater,
    GraterEqual,
    Less,
    LessEqual,
    Identifier(String),
    String(String),
    Number(f32),
    And,
    Or,
    If,
    Else,
    Is,
    Not,
    Mod,
    Fun,
    Loop,
    Reverse,
    As,
    Nil,
    Print,
    Return,
    True,
    False,
    Var,
    Const,
    While,
    Break,
    Eof,
    Colon
}

impl From<TokenType> for Operator {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::Mod => Operator::Mod,
            TokenType::And => Operator::And,
            TokenType::Slash => Operator::Div,
            TokenType::Star => Operator::Mul,
            _ => todo!()
        }
    }
}
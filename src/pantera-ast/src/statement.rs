use crate::expression::{Expression, Identifier};

pub enum GlobalStatement {
    Statement(Statement),
    FunctionDeclaration(FunctionDeclarationStatement)
}

pub enum Statement {
    Print(Box<PrintStatement>),
    Break,
    Block(Box<BlockStatement>),
    Expression(Box<ExpressionStatement>),
    Return(Box<ReturnStatement>),
    If(Box<IfStatement>),
    Declaration(DeclarationStatement),
    Loop(Box<LoopStatement>)
}

pub struct PrintStatement {
    expr: Expression
}

pub struct BlockStatement {
    statements: Vec<Statement>
}

pub struct ExpressionStatement {
    expr: Expression
}

pub struct ReturnStatement {
    value: Option<Expression>
}

pub struct IfStatement {
    condition: Expression,
    body: Statement,
    alternative: Option<Statement>
}

pub struct DeclarationStatement {
    kind: DeclarationKind,
    variable: Identifier,
    value: Option<Expression>
}

pub enum DeclarationKind {
    Var,
    Const
}

pub struct LoopStatement {
    body: Statement,
    range: Option<Range>,
    alias: Option<Identifier>
}

pub struct Range {
    start: Expression,
    stop: Expression
}

pub struct FunctionDeclarationStatement {
    name: Identifier,
    params: Vec<Identifier>,
    body: Statement
}

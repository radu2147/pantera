use crate::expression::{Expression, Identifier};
use crate::statement_visitor::StatementVisitorMut;

#[derive(Debug)]
pub enum GlobalStatement {
    Statement(Statement),
    FunctionDeclaration(FunctionDeclarationStatement)
}

#[derive(Debug)]
pub enum Statement {
    Print(Box<PrintStatement>),
    Break,
    Block(Box<BlockStatement>),
    Expression(Box<ExpressionStatement>),
    Return(Box<ReturnStatement>),
    If(Box<IfStatement>),
    Declaration(DeclarationStatement),
    MultiDeclaration(MultiDeclarationStatement),
    Loop(Box<LoopStatement>)
}

#[derive(Debug)]
pub struct PrintStatement {
    pub expr: Expression
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expr: Expression
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Option<Expression>
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Statement,
    pub alternative: Option<Statement>
}

#[derive(Debug)]
pub struct MultiDeclarationStatement {
    pub declarations: Vec<DeclarationStatement>
}

#[derive(Debug)]
pub struct DeclarationStatement {
    pub kind: DeclarationKind,
    pub variable: String,
    pub value: Option<Expression>
}

#[derive(Clone, Debug)]
pub enum DeclarationKind {
    Var,
    Const
}

#[derive(Debug)]
pub struct LoopStatement {
    pub body: Statement,
    pub alias: String
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start: Expression,
    pub stop: Option<Expression>
}

#[derive(Debug)]
pub struct FunctionDeclarationStatement {
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub body: Statement
}

impl GlobalStatement {
    pub fn visit<T: StatementVisitorMut>(&self, visitor: &mut T) {
        visitor.visit_statement(self);
    }
}
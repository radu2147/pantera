use crate::expression::{Expression, Identifier};
use crate::statement_visitor::{IntoStatementVisitorMut, StatementVisitorMut};

#[derive(Debug)]
pub enum GlobalStatement {
    Statement(Statement),
    FunctionDeclaration(FunctionDeclarationStatement)
}

#[derive(Debug)]
pub enum Statement {
    Print(Box<PrintStatement>),
    FunctionBody(Box<BlockStatement>),
    Break,
    Block(Box<BlockStatement>),
    Expression(Box<ExpressionStatement>),
    Return(Box<ReturnStatement>),
    If(Box<IfStatement>),
    Declaration(DeclarationStatement),
    MultiDeclaration(MultiDeclarationStatement),
    Loop(Box<LoopStatement>)
}

#[macro_export]
macro_rules! break_ {
    () => {
        Statement::Break
    };
}

#[macro_export]
macro_rules! fun_body {
    { $($body:tt)* } => {
        Statement::FunctionBody(Box::from(BlockStatement { $($body)* }))
    };
}

#[derive(Debug)]
pub struct PrintStatement {
    pub expr: Expression
}

#[macro_export]
macro_rules! print_ {
    { $($body:tt)* } => {
        Statement::Print(Box::from(PrintStatement { $($body)* }))
    };
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
}

#[macro_export]
macro_rules! block {
    { $($body:tt)* } => {
        Statement::Block(Box::from(BlockStatement { $($body)* }))
    };
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expr: Expression
}

#[macro_export]
macro_rules! expression {
    { $($body:tt)* } => {
        Statement::Expression(Box::from(ExpressionStatement { $($body)* }))
    };
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Option<Expression>
}

#[macro_export]
macro_rules! return_ {
    { $($body:tt)* } => {
        Statement::Return(Box::from(ReturnStatement { $($body)* }))
    };
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Statement,
    pub alternative: Option<Statement>
}

#[macro_export]
macro_rules! if_ {
    { $($body:tt)* } => {
        Statement::If(Box::from(IfStatement { $($body)* }))
    };
}

#[derive(Debug)]
pub struct MultiDeclarationStatement {
    pub declarations: Vec<DeclarationStatement>
}

#[macro_export]
macro_rules! multi_declaration {
     { $($body:tt)* } => {
        Statement::MultiDeclaration(MultiDeclarationStatement { $($body)* })
    };
 }

#[derive(Debug)]
pub struct DeclarationStatement {
    pub kind: DeclarationKind,
    pub variable: String,
    pub value: Option<Expression>
}

#[macro_export]
macro_rules! declaration {
     { $($body:tt)* } => {
        Statement::Declaration(DeclarationStatement { $($body)* })
    };
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

#[macro_export]
macro_rules! loop_ {
     { $($body:tt)* } => {
        Statement::Loop(Box::from(LoopStatement { $($body)* }))
    };
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

#[macro_export]
macro_rules! fun_declaration {
    { $($body:tt)* } => {
        GlobalStatement::FunctionDeclaration(FunctionDeclarationStatement { $($body)* })
    };
}

impl GlobalStatement {
    pub fn visit<T: StatementVisitorMut>(&self, visitor: &mut T) {
        visitor.visit_statement(self);
    }

    pub fn visit_g<T: IntoStatementVisitorMut>(self, visitor: &mut T) {
        visitor.visit_statement(self);
    }
}
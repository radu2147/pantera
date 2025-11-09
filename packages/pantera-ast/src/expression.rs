use crate::expression_visitor::ExpressionVisitorMut;

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Group(Box<GroupExpression>),
    Call(Box<CallExpression>),
    Member(Box<MemberExpression>),
    Nil,
    Bool(bool),
    Number(f32),
    String(String),
    Identifier(String),
    Object(Box<ObjectExpression>),
    Array(Box<ArrayExpression>),
    Assigment(Box<AssignmentExpression>)
}

#[macro_export]
macro_rules! nil {
    {} => {
        Expression::Nil
    };
}

#[macro_export]
macro_rules! bool_ {
    ($body:expr) => {
        Expression::Bool($body)
    };
}

#[macro_export]
macro_rules! identifier {
    ($body:expr) => {
        Expression::Identifier($body)
    };
}

#[macro_export]
macro_rules! number {
    ($body:expr) => {
        Expression::Number($body)
    };
}

#[macro_export]
macro_rules! string {
    ($body:expr) => {
        Expression::String($body)
    };
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub id: f32
}

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub assignee: Expression,
    pub value: Expression
}

#[macro_export]
macro_rules! assignment {
    { $($body:tt)* } => {
        Expression::Assigment(Box::from(AssignmentExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub struct ObjectExpression {
    pub properties: Vec<Expression>,
    pub values: Vec<Expression>
}

#[macro_export]
macro_rules! object {
    { $($body:tt)* } => {
        Expression::Object(Box::from(ObjectExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub struct ArrayExpression {
    pub values: Vec<Expression>
}

#[macro_export]
macro_rules! array {
    { $($body:tt)* } => {
        Expression::Array(Box::from(ArrayExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub struct MemberExpression {
    pub callee: Expression,
    pub property: Expression
}

#[macro_export]
macro_rules! member {
    { $($body:tt)* } => {
        Expression::Member(Box::from(MemberExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub callee: Expression,
    pub args: Vec<Expression>
}

#[macro_export]
macro_rules! call {
    { $($body:tt)* } => {
        Expression::Call(Box::from(CallExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: Operator,
    pub right: Expression
}

#[macro_export]
macro_rules! binary {
    { $($body:tt)* } => {
        Expression::Binary(Box::from(BinaryExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Operator,
    pub expr: Expression
}

#[macro_export]
macro_rules! unary {
    { $($body:tt)* } => {
        Expression::Unary(Box::from(UnaryExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub struct GroupExpression {
    pub expr: Expression
}

#[macro_export]
macro_rules! group {
    { $($body:tt)* } => {
        Expression::Group(Box::from(GroupExpression { $($body)* }))
    };
}

#[derive(Debug, Clone)]
pub enum Operator {
    And,
    Or,
    Ge,
    Le,
    Eq,
    NE,
    Greater,
    Less,
    Plus,
    Minus,
    Pow,
    Mul,
    Mod,
    Div
}

impl Expression {
    pub fn get_identifier(&self) -> Option<&String> {
        match self {
            Expression::Identifier(val) => Some(val),
            _ => None
        }
    }

    pub fn get_number(&self) -> Option<f32> {
        match self {
            Expression::Number(val) => Some(*val),
            _ => None
        }
    }

    pub fn visit<T: ExpressionVisitorMut>(&self, visitor: &mut T) {
        visitor.visit_expression(self);
    }
}
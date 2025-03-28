use crate::expression_visitor::ExpressionVisitorMut;

#[derive(Debug)]
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
    Assigment(Box<AssignmentExpression>)
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub id: f32
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub assignee: Expression,
    pub value: Expression
}

#[derive(Debug)]
pub struct ObjectExpression {
    pub properties: Vec<Expression>,
    pub values: Vec<Expression>
}

#[derive(Debug)]
pub struct MemberExpression {
    pub callee: Expression,
    pub property: Expression
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Expression,
    pub args: Vec<Expression>
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: Operator,
    pub right: Expression
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Operator,
    pub expr: Expression
}

#[derive(Debug)]
pub struct GroupExpression {
    pub expr: Expression
}

#[derive(Debug)]
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
    Div
}

impl Expression {
    pub fn get_identifier(&self) -> Option<&String> {
        match self {
            Expression::Identifier(val) => Some(val),
            _ => None
        }
    }

    pub fn visit<T: ExpressionVisitorMut>(&self, visitor: &mut T) {
        visitor.visit_expression(self);
    }
}
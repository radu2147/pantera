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
    Identifier(Identifier),
    Object(Box<ObjectExpression>),
    Assigment(Box<AssignmentExpression>)
}

pub struct Identifier {
    name: String,
    id: f32
}

pub struct AssignmentExpression {
    assignee: Identifier,
    value: Expression
}

pub struct ObjectExpression {
    properties: Vec<Expression>,
    values: Vec<Expression>
}

pub struct MemberExpression {
    callee: Expression,
    property: Expression
}

pub struct CallExpression {
    callee: Expression,
    args: Vec<Expression>
}

pub struct BinaryExpression {
    left: Expression,
    operator: Operator,
    right: Expression
}

pub struct UnaryExpression {
    operator: Operator,
    expr: Expression
}

pub struct GroupExpression {
    expr: Expression
}

pub enum Operator {
    And,
    Or,
    Ge,
    Le,
    Greater,
    Less,
    Plus,
    Minus,
    Pow,
    Mul,
    Div
}
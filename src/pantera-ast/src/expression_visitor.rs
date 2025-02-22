use crate::expression::{AssignmentExpression, BinaryExpression, CallExpression, Expression, GroupExpression, Identifier, MemberExpression, ObjectExpression, UnaryExpression};

pub trait ExpressionVisitor {
    fn visit_expression(&self, expression: &Expression) {
        match expression {
            Expression::Nil => self.visit_nil_expression(),
            Expression::Bool(ref value) => self.visit_boolean_expression(value),
            Expression::Number(ref value) => self.visit_number_expression(value),
            Expression::String(ref value) => self.visit_string_expression(value),
            Expression::Identifier(ref value) => self.visit_identifier_expression(value),
            Expression::Call(ref value) => self.visit_call_expression(value),
            Expression::Assigment(ref value) => self.visit_assignment_expression(value),
            Expression::Binary(ref value) => self.visit_binary_expression(value),
            Expression::Unary(ref value) => self.visit_unary_expression(value),
            Expression::Group(ref value) => self.visit_group_expression(value),
            Expression::Member(ref value) => self.visit_member_expression(value),
            Expression::Object(ref value) => self.visit_object_expression(value)
        }
    }

    fn visit_nil_expression(&self);
    fn visit_boolean_expression(&self, value: &bool);
    fn visit_number_expression(&self, value: &f32);
    fn visit_string_expression(&self, value: &String);
    fn visit_identifier_expression(&self, value: &Identifier);
    fn visit_call_expression(&self, value: &CallExpression);
    fn visit_assignment_expression(&self, value: &AssignmentExpression);
    fn visit_binary_expression(&self, value: &BinaryExpression);
    fn visit_unary_expression(&self, value: &UnaryExpression);
    fn visit_group_expression(&self, value: &GroupExpression);
    fn visit_member_expression(&self, value: &MemberExpression);
    fn visit_object_expression(&self ,value: &ObjectExpression);

}

pub trait ExpressionVisitorMut {
    fn visit_expression(&self, expression: &mut Expression) {
        match expression {
            Expression::Nil => self.visit_nil_expression(),
            Expression::Bool(ref mut value) => self.visit_boolean_expression(value),
            Expression::Number(ref mut value) => self.visit_number_expression(value),
            Expression::String(ref mut value) => self.visit_string_expression(value),
            Expression::Identifier(ref mut value) => self.visit_identifier_expression(value),
            Expression::Call(ref mut value) => self.visit_call_expression(value),
            Expression::Assigment(ref mut value) => self.visit_assignment_expression(value),
            Expression::Binary(ref mut value) => self.visit_binary_expression(value),
            Expression::Unary(ref mut value) => self.visit_unary_expression(value),
            Expression::Group(ref mut value) => self.visit_group_expression(value),
            Expression::Member(ref mut value) => self.visit_member_expression(value),
            Expression::Object(ref mut value) => self.visit_object_expression(value)
        }
    }

    fn visit_nil_expression(&self);
    fn visit_boolean_expression(&self, value: &mut bool);
    fn visit_number_expression(&self, value: &mut f32);
    fn visit_string_expression(&self, value: &mut String);
    fn visit_identifier_expression(&self, value: &mut Identifier);
    fn visit_call_expression(&self, value: &mut CallExpression);
    fn visit_assignment_expression(&self, value: &mut AssignmentExpression);
    fn visit_binary_expression(&self, value: &mut BinaryExpression);
    fn visit_unary_expression(&self, value: &mut UnaryExpression);
    fn visit_group_expression(&self, value: &mut GroupExpression);
    fn visit_member_expression(&self, value: &mut MemberExpression);
    fn visit_object_expression(&self ,value: &mut ObjectExpression);

}
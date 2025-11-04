use std::collections::HashMap;
use std::sync::Arc;
use pantera_ast::expression::{ArrayExpression, AssignmentExpression, BinaryExpression, CallExpression, Expression, MemberExpression, ObjectExpression, UnaryExpression};
use pantera_ast::expression_visitor::ExpressionVisitorMut;
use pantera_ast::statement::{DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, IfStatement, LoopStatement, PrintStatement, ReturnStatement};
use pantera_ast::statement_visitor::StatementVisitorMut;
use crate::errors::CompilerError;
use crate::semantic::check::Check;

pub struct DeclarationCheck {
    pub errors: Vec<CompilerError>,
    std_lib: Arc<HashMap<String, u16>>,
}

impl DeclarationCheck {
    pub fn new(std_lib: Arc<HashMap<String, u16>>) -> Self {
        Self {
            errors: vec![],
            std_lib
        }
    }
}

impl Check for DeclarationCheck {
    fn get_errors(self) -> Vec<CompilerError> {
        self.errors
    }
}

impl StatementVisitorMut for DeclarationCheck {

    fn visit_function_declaration(&mut self, func_dec: &FunctionDeclarationStatement) {
        self.visit_local_statement(&func_dec.body);
    }

    fn visit_break_statement(&mut self) {}

    fn visit_print_statement(&mut self, stmt: &PrintStatement) {
        self.visit_expression(&stmt.expr);
    }

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) {
        self.visit_expression(&stmt.expr);
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement) {
        if let Some(val) = &stmt.value {
            self.visit_expression(val);
        }
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) {
        self.visit_expression(&stmt.condition);
        self.visit_local_statement(&stmt.body);
    }

    fn visit_loop_statement(&mut self, stmt: &LoopStatement) {
        self.visit_local_statement(&stmt.body);
    }

    fn visit_declaration_statement(&mut self, stmt: &DeclarationStatement) {
        if self.std_lib.contains_key(&stmt.variable) {
            self.errors.push(CompilerError{ message: "Cannot declare a variable with a name from std lib".to_string() });
        }
    }
}

impl ExpressionVisitorMut for DeclarationCheck {
    fn visit_nil_expression(&mut self) {}

    fn visit_boolean_expression(&mut self, _value: &bool) {}

    fn visit_number_expression(&mut self, _value: &f32) {}

    fn visit_string_expression(&mut self, _value: &String) {}

    fn visit_identifier_expression(&mut self, _value: &String) {}

    fn visit_call_expression(&mut self, value: &CallExpression) {
        self.visit_expression(&value.callee);
        value.args.iter().for_each(|expr|{
            self.visit_expression(expr);
        })
    }

    fn visit_assignment_expression(&mut self, value: &AssignmentExpression) {
        match &value.assignee {
            Expression::Identifier(ident) => {
                if self.std_lib.contains_key(ident) {
                    self.errors.push(CompilerError{ message: "Cannot reassign a variable with name from std lib".to_string() });
                }
            }
            _ => {

            }
        }
    }

    fn visit_binary_expression(&mut self, value: &BinaryExpression) {
        self.visit_expression(&value.left);
        self.visit_expression(&value.right);
    }

    fn visit_unary_expression(&mut self, value: &UnaryExpression) {
        self.visit_expression(&value.expr);
    }

    fn visit_member_expression(&mut self, value: &MemberExpression) {
        self.visit_expression(&value.callee);
        self.visit_expression(&value.property);
    }

    fn visit_object_expression(&mut self, value: &ObjectExpression) {
        value.properties.iter().for_each(|val|{
            self.visit_expression(val);
        });
        value.values.iter().for_each(|val|{
            self.visit_expression(val);
        });
    }

    fn visit_array_expression(&mut self, value: &ArrayExpression) {
        value.values.iter().for_each(|val|{
            self.visit_expression(val);
        })
    }
}
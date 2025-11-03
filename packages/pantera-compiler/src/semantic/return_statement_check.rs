use pantera_ast::statement::{DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, IfStatement, LoopStatement, PrintStatement, ReturnStatement};
use pantera_ast::statement_visitor::StatementVisitorMut;
use crate::errors::CompilerError;
use crate::semantic::check::Check;

pub struct ReturnStatementCheck {
    pub errors: Vec<CompilerError>,
    pub is_function: bool
}

impl ReturnStatementCheck {
    pub fn new() -> Self {
        Self {
            errors: vec![],
            is_function: false,
        }
    }
}

impl Check for ReturnStatementCheck {
    fn get_errors(self) -> Vec<CompilerError> {
        self.errors
    }
}

impl StatementVisitorMut for ReturnStatementCheck {

    fn visit_function_declaration(&mut self, func_dec: &FunctionDeclarationStatement) {
        self.is_function = true;
        self.visit_local_statement(&func_dec.body);
        self.is_function = false;
    }

    fn visit_break_statement(&mut self) {}

    fn visit_print_statement(&mut self, _stmt: &PrintStatement) {}

    fn visit_expression_statement(&mut self, _stmt: &ExpressionStatement) {}

    fn visit_return_statement(&mut self, _stmt: &ReturnStatement) {
        if !self.is_function{
            self.errors.push(CompilerError {message: "Cannot return outside function".to_string()})
        }
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) {
        self.visit_local_statement(&stmt.body);
    }

    fn visit_loop_statement(&mut self, stmt: &LoopStatement) {
        self.visit_local_statement(&stmt.body);
    }

    fn visit_declaration_statement(&mut self, _stmt: &DeclarationStatement) {}
}
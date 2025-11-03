use pantera_ast::statement::{DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, IfStatement, LoopStatement, PrintStatement, ReturnStatement};
use pantera_ast::statement_visitor::StatementVisitorMut;
use crate::errors::CompilerError;
use crate::semantic::check::Check;

pub struct BreakStatementCheck {
    pub errors: Vec<CompilerError>,
    pub is_loop: bool
}

impl BreakStatementCheck {
    pub fn new() -> Self {
        Self {
            errors: vec![],
            is_loop: false
        }
    }
}

impl Check for BreakStatementCheck {
    fn get_errors(self) -> Vec<CompilerError> {
        self.errors
    }
}

impl StatementVisitorMut for BreakStatementCheck {

    fn visit_function_declaration(&mut self, func_dec: &FunctionDeclarationStatement) {
        self.visit_local_statement(&func_dec.body);
    }

    fn visit_break_statement(&mut self) {
        if !self.is_loop{
            self.errors.push(CompilerError{ message: "Break statement outside loop is not allowed".to_string() });
        }
    }

    fn visit_print_statement(&mut self, _stmt: &PrintStatement) {}

    fn visit_expression_statement(&mut self, _stmt: &ExpressionStatement) {}

    fn visit_return_statement(&mut self, _stmt: &ReturnStatement) {}

    fn visit_if_statement(&mut self, stmt: &IfStatement) {
        self.visit_local_statement(&stmt.body);
    }

    fn visit_loop_statement(&mut self, stmt: &LoopStatement) {
        self.is_loop = true;
        self.visit_local_statement(&stmt.body);
        self.is_loop = false;
    }

    fn visit_declaration_statement(&mut self, _stmt: &DeclarationStatement) {}
}
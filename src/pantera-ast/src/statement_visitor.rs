use crate::statement::{BlockStatement, DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, GlobalStatement, IfStatement, LoopStatement, MultiDeclarationStatement, PrintStatement, ReturnStatement, Statement};

pub trait StatementVisitor {
    fn visit_statement(&self, stmt: &GlobalStatement) {
        match stmt {
            GlobalStatement::Statement(ref value) => {
                match value {
                    Statement::Break => self.visit_break_statement(),
                    Statement::Print(ref value) => self.visit_print_statement(value),
                    Statement::Block(ref value) => self.visit_block_statement(value),
                    Statement::Expression(ref value) => self.visit_expression_statement(value),
                    Statement::Return(ref value) => self.visit_return_statement(value),
                    Statement::If(ref value) => self.visit_if_statement(value),
                    Statement::Declaration(ref value) => self.visit_declaration_statement(value),
                    Statement::Loop(ref value) => self.visit_loop_statement(value),
                    Statement::MultiDeclaration(ref value) => self.visit_multi_declaration(value)
                }
            }
            GlobalStatement::FunctionDeclaration(ref value) => self.visit_function_declaration(value)
        }
    }

    fn visit_multi_declaration(&self, stmt: &MultiDeclarationStatement);
    fn visit_function_declaration(&self, func_dec: &FunctionDeclarationStatement);
    fn visit_break_statement(&self);
    fn visit_print_statement(&self, stmt: &PrintStatement);
    fn visit_block_statement(&self, stmt: &BlockStatement);
    fn visit_expression_statement(&self, stmt: &ExpressionStatement);
    fn visit_return_statement(&self, stmt: &ReturnStatement);
    fn visit_if_statement(&self, stmt: &IfStatement);
    fn visit_loop_statement(&self, stmt: &LoopStatement);
    fn visit_declaration_statement(&self, stmt: &DeclarationStatement);
}

pub trait StatementVisitorMut {
    fn visit_statement(&mut self, stmt: &GlobalStatement) {
        match stmt {
            GlobalStatement::Statement(ref value) => {
                match value {
                    Statement::Break => self.visit_break_statement(),
                    Statement::Print(ref value) => self.visit_print_statement(value),
                    Statement::Block(ref value) => self.visit_block_statement(value),
                    Statement::Expression(ref value) => self.visit_expression_statement(value),
                    Statement::Return(ref value) => self.visit_return_statement(value),
                    Statement::If(ref value) => self.visit_if_statement(value),
                    Statement::Declaration(ref value) => self.visit_declaration_statement(value),
                    Statement::MultiDeclaration(ref value ) => self.visit_multi_declaration(value),
                    Statement::Loop(ref value) => self.visit_loop_statement(value),
                }
            }
            GlobalStatement::FunctionDeclaration(ref value) => self.visit_function_declaration(value)
        }
    }

    fn visit_function_declaration(&mut self, func_dec: &FunctionDeclarationStatement);
    fn visit_break_statement(&mut self);
    fn visit_print_statement(&mut self, stmt: &PrintStatement);
    fn visit_block_statement(&mut self, stmt: &BlockStatement);
    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement);
    fn visit_return_statement(&mut self, stmt: &ReturnStatement);
    fn visit_if_statement(&mut self, stmt: &IfStatement);
    fn visit_loop_statement(&mut self, stmt: &LoopStatement);
    fn visit_declaration_statement(&mut self, stmt: &DeclarationStatement);
    fn visit_multi_declaration(&mut self, stmt: &MultiDeclarationStatement);
}
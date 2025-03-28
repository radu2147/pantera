use pantera_ast::expression::{AssignmentExpression, BinaryExpression, CallExpression, GroupExpression, MemberExpression, ObjectExpression, Operator, UnaryExpression};
use pantera_ast::expression_visitor::ExpressionVisitorMut;
use pantera_ast::statement::{BlockStatement, DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, IfStatement, LoopStatement, MultiDeclarationStatement, PrintStatement, ReturnStatement};
use pantera_ast::statement_visitor::StatementVisitorMut;
use pantera_parser::parser::Parser;
use crate::bytecode::{Bytecode, OP_ADD, OP_DIV, OP_GET, OP_MUL, OP_PRINT, OP_SUB};
use crate::types::Type;

pub struct Compiler {
    pub code: Vec<Bytecode>
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            code: vec![]
        }
    }
    pub fn compile(&mut self, mut parser: Parser) {
        let program = parser.parse_program().unwrap();
        program.into_iter().for_each(|p| p.visit(self));
    }
}

impl Compiler {
    pub fn emit_byte(&mut self, byte_code: Bytecode) {
        self.code.push(byte_code);
    }

    pub fn emit_bytes(&mut self, byte_code1: Bytecode, byte_code2: Bytecode) {
        self.code.push(byte_code1);
        self.code.push(byte_code2);
    }

    pub fn emit_number(&mut self, number: f32) {
        self.emit_byte(OP_GET);
        self.emit_byte(Type::Number.into());
        self.convert_number_to_bytes(number).into_iter().for_each(|bc| self.emit_byte(bc));
    }

    pub fn emit_boolean(&mut self, val: bool) {
        self.emit_byte(OP_GET);
        self.emit_byte(Type::Boolean.into());
        self.emit_byte(self.convert_bool_to_byte(val));
    }

    pub fn emit_null(&mut self) {
        self.emit_bytes(OP_GET, Type::Null.into());
    }

    pub fn convert_number_to_bytes(&self, number: f32) -> [u8;4] {
        number.to_le_bytes()
    }

    pub fn convert_bool_to_byte(&self, val: bool) -> u8 {
        if val{
            return 1;
        }
        0
    }

    pub fn convert_bool_from_byte(&self, val: u8) -> bool {
        val == 1
    }

    pub fn convert_number_from_bytes(&self, number: [u8; 4]) -> f32 {
        f32::from_le_bytes(number)
    }
}

impl ExpressionVisitorMut for Compiler {
    fn visit_nil_expression(&mut self) {
        self.emit_null();
    }

    fn visit_boolean_expression(&mut self, value: &bool) {
        self.emit_boolean(*value);
    }

    fn visit_number_expression(&mut self, value: &f32) {
        self.emit_number(*value);
    }

    fn visit_string_expression(&mut self, value: &String) {
        todo!()
    }

    fn visit_identifier_expression(&mut self, value: &String) {
        todo!()
    }

    fn visit_call_expression(&mut self, value: &CallExpression) {
        todo!()
    }

    fn visit_assignment_expression(&mut self, value: &AssignmentExpression) {
        todo!()
    }

    fn visit_binary_expression(&mut self, value: &BinaryExpression) {
        self.visit_expression(&value.left);
        self.visit_expression(&value.right);
        match &value.operator {
            Operator::Plus => {
                self.emit_byte(OP_ADD);
            },
            Operator::Minus => {
                self.emit_byte(OP_SUB);
            },
            Operator::Div => {
                self.emit_byte(OP_DIV);
            },
            Operator::Mul => {
                self.emit_byte(OP_MUL);
            }
            _ => {
                todo!()
            }
        }
    }

    fn visit_unary_expression(&mut self, value: &UnaryExpression) {
        todo!()
    }

    fn visit_group_expression(&mut self, value: &GroupExpression) {
        self.visit_expression(&value.expr);
    }

    fn visit_member_expression(&mut self, value: &MemberExpression) {
        todo!()
    }

    fn visit_object_expression(&mut self, value: &ObjectExpression) {
        todo!()
    }
}

impl StatementVisitorMut for Compiler {
    fn visit_function_declaration(&mut self, func_dec: &FunctionDeclarationStatement) {
        todo!()
    }

    fn visit_break_statement(&mut self) {
        todo!()
    }

    fn visit_print_statement(&mut self, stmt: &PrintStatement) {
        self.visit_expression(&stmt.expr);
        self.emit_byte(OP_PRINT);
    }

    fn visit_block_statement(&mut self, stmt: &BlockStatement) {
        todo!()
    }

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) {
        todo!()
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement) {
        todo!()
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) {
        todo!()
    }

    fn visit_loop_statement(&mut self, stmt: &LoopStatement) {
        todo!()
    }

    fn visit_declaration_statement(&mut self, stmt: &DeclarationStatement) {
        todo!()
    }

    fn visit_multi_declaration(&mut self, stmt: &MultiDeclarationStatement) {
        todo!()
    }
}
use pantera_ast::expression::{AssignmentExpression, BinaryExpression, CallExpression, GroupExpression, MemberExpression, ObjectExpression, Operator, UnaryExpression};
use pantera_ast::expression_visitor::ExpressionVisitorMut;
use pantera_ast::statement::{BlockStatement, DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, IfStatement, LoopStatement, MultiDeclarationStatement, PrintStatement, ReturnStatement};
use pantera_ast::statement_visitor::StatementVisitorMut;
use pantera_parser::parser::Parser;
use crate::bytecode::{Bytecode, OP_ADD, OP_DIV, OP_PUSH, OP_MUL, OP_POW, OP_PRINT, OP_SUB, OP_EQ, OP_NE, OP_AND, OP_OR, OP_GE, OP_LE, OP_GR, OP_LS, OP_UNARY_SUB, OP_UNARY_NOT, OP_POP, OP_DECLARE, OP_GET, OP_SET};
use crate::env::Env;
use crate::types::Type;

pub struct Compiler {
    pub code: Vec<Bytecode>,
    pub env: Box<Env>
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            code: vec![],
            env: Box::new(Env::new())
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
        self.emit_byte(OP_PUSH);
        self.emit_byte(Type::Number.into());
        self.convert_number_to_bytes(number).into_iter().for_each(|bc| self.emit_byte(bc));
    }

    pub fn emit_boolean(&mut self, val: bool) {
        self.emit_byte(OP_PUSH);
        self.emit_byte(Type::Boolean.into());
        self.emit_byte(self.convert_bool_to_byte(val));
    }

    pub fn emit_null(&mut self) {
        self.emit_bytes(OP_PUSH, Type::Null.into());
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
        let var = self.env.get_variable(value);
        if var.is_some() {
            let value = *var.unwrap();
            self.emit_byte(OP_PUSH);
            self.emit_bytes(OP_GET, value)
        } else {
            panic!("Variable doesn't exist");
        }
    }

    fn visit_call_expression(&mut self, value: &CallExpression) {
        todo!()
    }

    fn visit_assignment_expression(&mut self, value: &AssignmentExpression) {
        self.visit_expression(&value.value);
        let var = self.env.get_variable(&value.assignee);
        if var.is_some() {
            self.emit_bytes(OP_SET, *var.unwrap());
        } else {
            panic!("Variable must be declared first");
        }
    }

    fn visit_binary_expression(&mut self, value: &BinaryExpression) {
        self.visit_expression(&value.left);
        self.visit_expression(&value.right);
        match &value.operator {
            Operator::Plus => self.emit_byte(OP_ADD),
            Operator::Minus => self.emit_byte(OP_SUB),
            Operator::Div => self.emit_byte(OP_DIV),
            Operator::Mul => self.emit_byte(OP_MUL),
            Operator::Pow => self.emit_byte(OP_POW),
            Operator::Eq => self.emit_byte(OP_EQ),
            Operator::NE => self.emit_byte(OP_NE),
            Operator::And => self.emit_byte(OP_AND),
            Operator::Or => self.emit_byte(OP_OR),
            Operator::Ge => self.emit_byte(OP_GE),
            Operator::Le => self.emit_byte(OP_LE),
            Operator::Greater => self.emit_byte(OP_GR),
            Operator::Less => self.emit_byte(OP_LS)
        }
    }

    fn visit_unary_expression(&mut self, value: &UnaryExpression) {
        self.visit_expression(&value.expr);
        match &value.operator {
            Operator::Minus => self.emit_byte(OP_UNARY_SUB),
            Operator::NE => self.emit_byte(OP_UNARY_NOT),
            _ => panic!("Other unary operator not supported")
        }
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
        self.env = Box::new(Env::new_local(self.env.clone()));

        stmt.statements.iter().for_each(|stmt| self.visit_local_statement(stmt));

        for _ in 0..self.env.variables.len() {
            self.emit_byte(OP_POP);
        }
        self.env = self.env.enclosing.clone().unwrap();
    }

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) {
        self.visit_expression(&stmt.expr);
        self.emit_byte(OP_POP);
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
        if let Some(ref val) = stmt.value {
            self.visit_expression(val);
            self.env.set_variable(stmt.variable.clone());
        } else {
            self.env.set_variable(stmt.variable.clone());
            self.emit_byte(OP_DECLARE);
        }
    }

    fn visit_multi_declaration(&mut self, stmt: &MultiDeclarationStatement) {
        for stmt in &stmt.declarations {
            self.visit_declaration_statement(stmt);
        }
    }
}
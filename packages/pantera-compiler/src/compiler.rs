use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use pantera_ast::expression::{ArrayExpression, AssignmentExpression, BinaryExpression, CallExpression, Expression, GroupExpression, MemberExpression, ObjectExpression, Operator, UnaryExpression};
use pantera_ast::expression_visitor::{IntoExpressionVisitorMut};
use pantera_ast::statement::{BlockStatement, DeclarationKind, DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, IfStatement, LoopStatement, MultiDeclarationStatement, PrintStatement, ReturnStatement};
use pantera_ast::statement_visitor::{IntoStatementVisitorMut};
use pantera_parser::parser::Parser;
use crate::bytecode::{Bytecode, OP_ADD, OP_DIV, OP_PUSH, OP_MUL, OP_POW, OP_PRINT, OP_SUB, OP_EQ, OP_NE, OP_AND, OP_OR, OP_GE, OP_LE, OP_GR, OP_LS, OP_UNARY_SUB, OP_UNARY_NOT, OP_POP, OP_DECLARE, OP_GET, OP_SET, OP_JUMP_IF_FALSE, OP_JUMP, OP_DECLARE_GLOBAL, OP_GET_GLOBAL, OP_SET_GLOBAL, OP_END_FUNCTION, OP_CALL, OP_RETURN, OP_ALLOCATE, OP_ACCESS, OP_SET_PROPERTY, OP_ALLOCATE_ARRAY};
use crate::env::Env;
use pantera_heap::heap::HeapManager;
use pantera_heap::types::Type;
use pantera_std::init_compiler_globals;
use crate::semantic::run_all_semantic_checks;

#[derive(Debug, Clone)]
pub enum Context {
    Global,
    Block,
    Function(String)
}

#[derive(Debug)]
pub struct Compiler {
    pub heap_manager: Rc<RefCell<HeapManager>>,
    pub code: Vec<Bytecode>,
    pub env: Box<Env>,
    pub break_stmt: Vec<Vec<usize>>,
    pub context: Context,
    pub globals: HashMap<String, u16>,
    pub active_func_args: HashMap<String, Vec<String>>,
}

impl Compiler {
    pub fn new(heap_manager: Rc<RefCell<HeapManager>>) -> Self {
        let std_lib = init_compiler_globals();
        Compiler {
            break_stmt: vec![],
            code: vec![],
            env: Box::new(Env::new()),
            context: Context::Global,
            globals: std_lib,
            active_func_args: HashMap::new(),
            heap_manager
        }
    }
    pub fn compile(mut self, mut parser: Parser) -> Result<Vec<Bytecode>, String> {
        match parser.parse_program() {
           Ok(program) => {
               run_all_semantic_checks(&program)?;
               program.into_iter().for_each(|p| p.visit_g(&mut self));

               Ok(self.code)
           }
            Err(e) => {
                Err(e.get_message())
            }
        }

    }
}

impl Compiler {
    pub(crate)fn emit_byte(&mut self, byte_code: Bytecode) {
        self.code.push(byte_code);
    }

    pub(crate)fn emit_bytes(&mut self, byte_code1: Bytecode, byte_code2: Bytecode) {
        self.code.push(byte_code1);
        self.code.push(byte_code2);
    }

    pub(crate)fn emit_number(&mut self, number: f32) {
        self.emit_byte(OP_PUSH);
        self.emit_byte(Type::Number as Bytecode);
        Self::convert_number_to_bytes(number).into_iter().for_each(|bc| self.emit_byte(bc));
    }

    pub(crate)fn emit_boolean(&mut self, val: bool) {
        self.emit_byte(OP_PUSH);
        self.emit_byte(Type::Boolean as Bytecode);
        self.emit_byte(Self::convert_bool_to_byte(val));
    }

    pub(crate)fn emit_null(&mut self) {
        self.emit_bytes(OP_PUSH, Type::Null as Bytecode);
    }

    pub(crate) fn emit_temp_byte(&mut self) {
        self.emit_byte(0);
        self.emit_byte(0);
        self.emit_byte(0);
        self.emit_byte(0);
    }

    pub(crate) fn emit_jump(&mut self) -> usize {
        self.emit_byte(OP_JUMP);
        let loc = self.code.len();

        self.emit_temp_byte();

        loc
    }

    pub(crate) fn emit_hash(&mut self, variable: String) {
        if let Some(key) = self.globals.get(&variable) {
            key.to_le_bytes().iter().for_each(|bt|self.emit_byte(*bt));
            return;
        }
        let val = self.globals.len() as u16;
        self.globals.insert(variable, val);
        val.to_le_bytes().into_iter().for_each(|bt| self.emit_byte(bt));
    }

    pub(crate) fn back_patch(&mut self, index: usize) {
        let numb = Self::convert_number_to_bytes(self.code.len() as f32);
        self.code[index] = numb[0];
        self.code[index + 1] = numb[1];
        self.code[index + 2] = numb[2];
        self.code[index + 3] = numb[3];
    }

    #[inline]
    pub(crate) fn convert_number_to_bytes(number: f32) -> [u8;4] {
        number.to_le_bytes()
    }

    pub(crate) fn convert_bool_to_byte(val: bool) -> u8 {
        if val{
            return 1;
        }
        0
    }

    #[inline]
    pub fn convert_bool_from_byte(val: u8) -> bool {
        val == 1
    }

    pub fn convert_number_from_bytes(number: [u8; 4]) -> f32 {
        f32::from_le_bytes(number)
    }
}

impl IntoExpressionVisitorMut for Compiler {
    fn visit_nil_expression(&mut self) {
        self.emit_null();
    }

    fn visit_boolean_expression(&mut self, value: bool) {
        self.emit_boolean(value);
    }

    fn visit_number_expression(&mut self, value: f32) {
        self.emit_number(value);
    }

    fn visit_string_expression(&mut self, value: String) {
        self.emit_bytes(OP_PUSH, Type::String as Bytecode);

        let ptr = self.heap_manager.borrow_mut().allocate_compiled_string(value).unwrap();
        (ptr as u64).to_le_bytes().into_iter().for_each(|bt| self.emit_byte(bt));
    }

    fn visit_identifier_expression(&mut self, value: String) {
        let var = self.env.get_variable(&value);
        if let Some(variable) = var {
            let value = variable.key;
            self.emit_byte(OP_PUSH);
            self.emit_bytes(OP_GET, value);
        } else {
            self.emit_byte(OP_PUSH);
            self.emit_byte(OP_GET_GLOBAL);
            self.emit_hash(value);
        }
    }

    fn visit_call_expression(&mut self, value: CallExpression) {
        value.args.into_iter().for_each(|arg| self.visit_expression(arg));
        self.visit_expression(value.callee);
        self.emit_byte(OP_CALL);
    }

    fn visit_assignment_expression(&mut self, value: AssignmentExpression) {
        self.visit_expression(value.value);
        match value.assignee {
            Expression::Identifier(ident) => {
                let var = self.env.get_variable(&ident);
                if let Some(variable) = var {
                    if variable.is_constant {
                        panic!("Cannot reassign a variable declared as const")
                    }
                    self.emit_bytes(OP_SET, variable.key);
                } else {
                    self.emit_byte(OP_SET_GLOBAL);
                    self.emit_hash(ident);
                }
            },
            Expression::Member(mem) => {
                self.visit_expression(mem.callee);
                self.visit_expression(mem.property);
                self.emit_byte(OP_SET_PROPERTY);
            }
            _ => {
                panic!("Unreachable");
            }
        }
    }

    fn visit_binary_expression(&mut self, value: BinaryExpression) {
        self.visit_expression(value.left);
        self.visit_expression(value.right);
        match value.operator {
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

    fn visit_unary_expression(&mut self, value: UnaryExpression) {
        self.visit_expression(value.expr);
        match value.operator {
            Operator::Minus => self.emit_byte(OP_UNARY_SUB),
            Operator::NE => self.emit_byte(OP_UNARY_NOT),
            _ => panic!("Other unary operator not supported")
        }
    }

    fn visit_group_expression(&mut self, value: GroupExpression) {
        self.visit_expression(value.expr);
    }

    fn visit_member_expression(&mut self, value: MemberExpression) {
        self.visit_expression(value.property);
        self.visit_expression(value.callee);
        self.emit_byte(OP_ACCESS);
    }

    fn visit_object_expression(&mut self, value: ObjectExpression) {
        let obj_len = value.properties.len() as f32;
        value.properties.into_iter().for_each(|prop| self.visit_expression(prop));
        value.values.into_iter().for_each(|val| self.visit_expression(val));
        self.emit_number(obj_len);
        self.emit_byte(OP_ALLOCATE);
    }

    fn visit_array_expression(&mut self, value: ArrayExpression) {
        let arr_len =value.values.len() as f32;
        value.values.into_iter().for_each(|val| self.visit_expression(val));
        self.emit_number(arr_len);
        self.emit_byte(OP_ALLOCATE_ARRAY);
    }
}

impl IntoStatementVisitorMut for Compiler {
    fn visit_function_body(&mut self, stmt: BlockStatement) {
        self.env = Box::new(Env::new_frame(self.env.clone()));
        self.env.set_variable("__offset__".to_string());
        let Context::Function(func_name) = &self.context else {panic!("Something went wrong when compiling")};
        self.active_func_args.get(func_name).unwrap().iter().for_each(|param| self.env.set_variable(param.clone()));

        stmt.statements.into_iter().for_each(|stmt| self.visit_local_statement(stmt));

        self.env = self.env.enclosing.clone().unwrap();
    }

    fn visit_function_declaration(&mut self, func_dec: FunctionDeclarationStatement) {
        let old_context = self.context.clone();
        self.context = Context::Function(func_dec.name.name.clone());
        self.emit_byte(OP_PUSH);
        self.emit_byte(Type::Function as Bytecode);
        let addr = self.code.len();
        self.emit_temp_byte();

        self.emit_byte(func_dec.params.len() as Bytecode);
        self.active_func_args.insert(func_dec.name.name.clone(), func_dec.params.into_iter().map(|param| param.name).collect::<Vec<String>>());

        self.emit_byte(OP_DECLARE_GLOBAL);
        self.emit_hash(func_dec.name.name);

        let loc = self.emit_jump();
        self.back_patch(addr);
        self.visit_local_statement(func_dec.body);
        self.emit_byte(OP_END_FUNCTION);

        self.back_patch(loc);

        self.context = old_context;
    }

    fn visit_break_statement(&mut self) {
        self.emit_byte(OP_JUMP);
        let cont_ind = self.break_stmt.len() - 1;
        if let Some(cont) = self.break_stmt.get_mut(cont_ind) {
            cont.push(self.code.len());
        }
        self.emit_temp_byte();
    }

    fn visit_print_statement(&mut self, stmt: PrintStatement) {
        self.visit_expression(stmt.expr);
        self.emit_byte(OP_PRINT);
    }

    fn visit_block_statement(&mut self, stmt: BlockStatement) {
        let old_context = self.context.clone();
        self.context = Context::Block;

        self.env = Box::new(Env::new_local(self.env.clone()));

        stmt.statements.into_iter().for_each(|stmt| self.visit_local_statement(stmt));

        for _ in 0..self.env.variables.len() {
            self.emit_byte(OP_POP);
        }
        self.env = self.env.enclosing.clone().unwrap();
        self.context = old_context;
    }

    fn visit_expression_statement(&mut self, stmt: ExpressionStatement) {
        self.visit_expression(stmt.expr);
        self.emit_byte(OP_POP);
    }

    fn visit_return_statement(&mut self, stmt: ReturnStatement) {
        if let Some(value) = stmt.value {
            self.visit_expression(value);
            self.emit_byte(OP_RETURN);
        }
        self.emit_byte(OP_END_FUNCTION);
    }

    fn visit_if_statement(&mut self, stmt: IfStatement) {
        self.visit_expression(stmt.condition);
        self.emit_byte(OP_JUMP_IF_FALSE);

        let loc = self.code.len();

        self.emit_temp_byte();

        self.visit_local_statement(stmt.body);
        if let Some(alt) = stmt.alternative {
            let loc_else = self.emit_jump();
            self.back_patch(loc);

            self.visit_local_statement(alt);

            self.back_patch(loc_else)
        } else {
            self.back_patch(loc);
        }
    }

    fn visit_loop_statement(&mut self, stmt: LoopStatement) {
        self.break_stmt.push(vec![]);

        let loc = self.code.len();
        self.visit_local_statement(stmt.body);
        self.emit_byte(OP_JUMP);
        let beg = Self::convert_number_to_bytes(loc as f32);
        for index in 0..4 {
            self.emit_byte(beg[index]);
        }

        let cont = self.break_stmt.pop().unwrap();
        cont.into_iter().for_each(|break_location| self.back_patch(break_location));
    }

    fn visit_declaration_statement(&mut self, stmt: DeclarationStatement) {
        if matches!(self.context, Context::Global) {
            if let Some(val) = stmt.value {
                self.visit_expression(val);
            } else {
                self.emit_null();
            }
            self.emit_byte(OP_DECLARE_GLOBAL);
            self.emit_hash(stmt.variable.clone());
        }
        else if let Some(val) = stmt.value {
            self.visit_expression(val);
            if matches!(stmt.kind, DeclarationKind::Const) {
                self.env.set_constant(stmt.variable.clone());
            } else {
                self.env.set_variable(stmt.variable.clone());
            }
        } else {
            if matches!(stmt.kind, DeclarationKind::Const) {
                panic!("Cannot declare a variable with no value as const");
            }
            self.env.set_variable(stmt.variable);
            self.emit_byte(OP_DECLARE);
        }
    }

    fn visit_multi_declaration(&mut self, stmt: MultiDeclarationStatement) {
        for stmt in stmt.declarations {
            self.visit_declaration_statement(stmt);
        }
    }
}
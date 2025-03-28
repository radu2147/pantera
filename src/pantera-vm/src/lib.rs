mod stack;
mod value;

use pantera_compiler::bytecode::Bytecode;
use pantera_compiler::compiler::Compiler;
use pantera_compiler::types::Type;
use pantera_compiler::bytecode::{OP_GET, OP_PRINT, OP_ADD, OP_SUB, OP_DIV, OP_MUL};
use crate::stack::Stack;
use crate::value::Value;

pub struct VM {
    compiler: Compiler,
    execution_stack: Stack<Value>,
    ip: usize
}

impl VM {
    fn read_constant(&mut self) -> Value {
        let typ = self.peek().unwrap().into();
        self.advance();
        match typ {
            Type::Null => Value::Null,
            Type::Boolean => {
                self.advance();
                let val = self.peek().unwrap();
                Value::Bool(self.compiler.convert_bool_from_byte(*val))
            },
            Type::Number => {
                let mut bytes: [Bytecode; 4] = [0, 0, 0, 0];
                for i in 0..4 {
                    bytes[i] = *self.peek().unwrap();
                    self.advance();
                }
                Value::Number(self.compiler.convert_number_from_bytes(bytes))
            }
        }
    }

    pub fn execute(&mut self) {
        while !self.is_at_end() {
            match *self.peek().unwrap() {
                OP_GET => {
                    self.advance();
                    let val = self.read_constant();
                    self.execution_stack.push(val);
                },
                OP_ADD => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Number(num1 + num2));
                                }
                                _ => panic!("Addition of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            todo!()
                        }
                    }
                },
                OP_SUB => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Number(num2 - num1));
                                }
                                _ => panic!("Addition of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            todo!()
                        }
                    }
                },
                OP_MUL => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Number(num1 * num2));
                                }
                                _ => panic!("Addition of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            todo!()
                        }
                    }
                },
                OP_DIV => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Number(num2 / num1));
                                }
                                _ => panic!("Addition of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            todo!()
                        }
                    }
                }
                OP_PRINT => {
                    self.advance();
                    let val = self.execution_stack.pop().unwrap();
                    println!("{val}");
                },
                _ => {
                    todo!();
                }
            }
        }
    }

    pub fn init(compiler: Compiler) -> Self {
        Self {
            compiler,
            execution_stack: Stack::<Value>::init(),
            ip: 0usize
        }
    }

    fn peek(&self) -> Option<&Bytecode> {
        self.compiler.code.get(self.ip)
    }

    fn advance(&mut self) {
        self.ip += 1;
    }

    fn is_at_end(&self) -> bool {
        self.ip == self.compiler.code.len()
    }
}

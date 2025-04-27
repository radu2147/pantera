mod stack;
mod value;

use std::collections::HashMap;
use pantera_compiler::bytecode::{Bytecode, OP_GET_GLOBAL};
use pantera_compiler::compiler::Compiler;
use pantera_compiler::types::Type;
use pantera_compiler::bytecode::{OP_PUSH, OP_PRINT, OP_RETURN, OP_END_FUNCTION, OP_JUMP, OP_JUMP_IF_FALSE, OP_ADD, OP_SUB, OP_POP, OP_DIV, OP_MUL, OP_POW, OP_EQ, OP_NE, OP_AND, OP_SET, OP_SET_GLOBAL, OP_OR, OP_GE, OP_GR, OP_LE, OP_LS, OP_UNARY_NOT, OP_UNARY_SUB, OP_GET, OP_DECLARE, OP_DECLARE_GLOBAL, OP_CALL};
use crate::stack::Stack;
use crate::value::Value;

pub struct VM {
    compiler: Compiler,
    execution_stack: Stack,
    ip: usize,
    globals: HashMap<u16, Value>
}

impl VM {
    fn read_constant(&mut self) -> Value {
        let typ = self.peek().unwrap().into();
        self.advance();
        match typ {
            Type::Null => Value::Null,
            Type::Boolean => {
                let val = *self.peek().unwrap();
                self.advance();
                Value::Bool(self.compiler.convert_bool_from_byte(val))
            },
            Type::Number => {
                let mut bytes: [Bytecode; 4] = [0, 0, 0, 0];
                for i in 0..4 {
                    bytes[i] = *self.peek().unwrap();
                    self.advance();
                }
                Value::Number(self.compiler.convert_number_from_bytes(bytes))
            },
            Type::Function => {
                let mut bytes: [Bytecode; 4] = [0, 0, 0, 0];
                for i in 0..4 {
                    bytes[i] = *self.peek().unwrap();
                    self.advance();
                }
                let arity = *self.peek().unwrap();
                self.advance();
                Value::Function(self.compiler.convert_number_from_bytes(bytes) as usize, arity)
            }
        }
    }

    fn pow_numbers(base: f32, pow: f32) -> f32 {
        if pow.fract() == 0.0 {
            let pw = pow as u32;
            if pw < 0 {
                return 1.0 / Self::power(base, pw);
            }
            Self::power(base, pw)
        } else {
            panic!("Pow being a float number is not supported")
        }
    }

    fn power(base: f32, pow: u32) -> f32 {
        if pow == 0 {
            return 1.0;
        }
        if pow == 1 {
            return base;
        }

        if pow % 2 == 0 {
            return Self::power(base, pow / 2) * Self::power(base, pow / 2);
        }

        Self::power(base, pow / 2) * Self::power(base, pow / 2) * base
    }

    pub fn read_global(&mut self) -> u16 {
        let mut var_key = [0u8; 2];
        for i in 0..2 {
            var_key[i] = *self.peek().unwrap();
            self.advance();
        }

        u16::from_le_bytes(var_key)
    }

    pub fn execute(&mut self) {
        while !self.is_at_end() {
            match *self.peek().unwrap() {
                OP_PUSH => {
                    self.advance();
                    let val = if *self.peek().unwrap() == OP_GET {
                        self.advance();
                        let var_key = *self.peek().unwrap() as usize;
                        let value = self.execution_stack.get(var_key).unwrap().clone();
                        self.advance();

                        value

                    } else if *self.peek().unwrap() == OP_GET_GLOBAL {
                        self.advance();
                        let var_key = self.read_global();

                        self.globals.get(&var_key).unwrap_or_else(|| {panic!("Variable doesn't exist")}).clone()
                    } else{
                        self.read_constant()
                    };

                    self.execution_stack.push(val);
                },
                OP_JUMP_IF_FALSE => {
                    self.advance();
                    let mut bytes: [u8;4] = [0;4];
                    for i in 0..4 {
                        bytes[i] = *self.peek().unwrap();
                        self.advance();
                    }

                    let num = self.compiler.convert_number_from_bytes(bytes) as usize;
                    let val = self.execution_stack.pop().unwrap();
                    if let Value::Bool(false) = val {
                        self.ip = num;
                    }
                },
                OP_JUMP => {
                    self.advance();
                    let mut bytes: [u8;4] = [0;4];
                    for i in 0..4 {
                        bytes[i] = *self.peek().unwrap();
                        self.advance();
                    }
                    let num = self.compiler.convert_number_from_bytes(bytes) as usize;
                    self.ip = num;
                }
                OP_ADD => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Number(num2 + num1));
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
                                    self.execution_stack.push(Value::Number(num2 * num1));
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
                },
                OP_POW => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Number(Self::pow_numbers(num2, num1)));
                                }
                                _ => panic!("Pow of variables of different types is not supported")
                            }
                        },
                        _ => {
                            panic!("Pow of anything but numbers is not supported")
                        }
                    }
                },
                OP_EQ => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Bool(num1 == num2));
                                }
                                _ => panic!("Equality of variables of different types is not supported")
                            }
                        },
                        Value::Bool(val1) => {
                            match val2 {
                                Value::Bool(val2) => {
                                    self.execution_stack.push(Value::Bool(val1 == val2));
                                }
                                _ => panic!("Equality of variables of different types is not supported")
                            }
                        },
                        Value::Null => {
                            match val2 {
                                Value::Null => {
                                    self.execution_stack.push(Value::Bool(true));
                                }
                                _ => {
                                    self.execution_stack.push(Value::Bool(false));
                                }
                            }
                        },
                        Value::Function(ip, _) => {
                            match val2 {
                                Value::Function(ip2, _) => {
                                    self.execution_stack.push(Value::Bool(ip == ip2));
                                },
                                _ => {
                                    self.execution_stack.push(Value::Bool(false));
                                }
                            }
                        }
                    }
                },
                OP_NE => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(num1) => {
                            match val2 {
                                Value::Number(num2) => {
                                    self.execution_stack.push(Value::Bool(num1 != num2));
                                }
                                _ => panic!("Equality of variables of different types is not supported")
                            }
                        },
                        Value::Bool(val1) => {
                            match val2 {
                                Value::Bool(val2) => {
                                    self.execution_stack.push(Value::Bool(val1 != val2));
                                }
                                _ => panic!("Equality of variables of different types is not supported")
                            }
                        },
                        Value::Null => {
                            match val2 {
                                Value::Null => {
                                    self.execution_stack.push(Value::Bool(false));
                                }
                                _ => {
                                    self.execution_stack.push(Value::Bool(true));
                                }
                            }
                        },
                        Value::Function(ip, _) => {
                            match val2 {
                                Value::Function(ip2, _) => {
                                    self.execution_stack.push(Value::Bool(ip != ip2));
                                },
                                _ => {
                                    self.execution_stack.push(Value::Bool(true));
                                }
                            }
                        }
                    }
                },
                OP_UNARY_NOT => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Bool(val1) => {
                            self.execution_stack.push(Value::Bool(!val1));
                        }
                        _ => {
                            panic!("Notting a non-boolean value is not allowed");
                        }
                    }
                },
                OP_UNARY_SUB => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(val1) => {
                            self.execution_stack.push(Value::Number(-val1));
                        }
                        _ => {
                            panic!("Minusing a non-number value is not allowed");
                        }
                    }
                }
                OP_AND => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Bool(val1) => {
                            match val2 {
                                Value::Bool(val2) => {
                                    self.execution_stack.push(Value::Bool(val1 && val2));
                                }
                                _ => panic!("And of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            panic!("And of anything but boolean variables not supported")
                        }
                    }
                },
                OP_OR => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Bool(val1) => {
                            match val2 {
                                Value::Bool(val2) => {
                                    self.execution_stack.push(Value::Bool(val1 || val2));
                                }
                                _ => panic!("And of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            panic!("And of anything but boolean variables not supported")
                        }
                    }
                },
                OP_GE => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(val1) => {
                            match val2 {
                                Value::Number(val2) => {
                                    self.execution_stack.push(Value::Bool(val2 >= val1));
                                }
                                _ => panic!("Comparison of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            panic!("Comparison of anything but numbers variables not supported")
                        }
                    }
                },
                OP_GR => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(val1) => {
                            match val2 {
                                Value::Number(val2) => {
                                    self.execution_stack.push(Value::Bool(val2 > val1));
                                }
                                _ => panic!("Comparison of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            panic!("Comparison of anything but numbers variables not supported")
                        }
                    }
                },
                OP_LE => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(val1) => {
                            match val2 {
                                Value::Number(val2) => {
                                    self.execution_stack.push(Value::Bool(val2 <= val1));
                                }
                                _ => panic!("Comparison of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            panic!("Comparison of anything but numbers variables not supported")
                        }
                    }
                },
                OP_LS => {
                    self.advance();
                    let val1 = self.execution_stack.pop().unwrap();
                    let val2 = self.execution_stack.pop().unwrap();
                    match val1 {
                        Value::Number(val1) => {
                            match val2 {
                                Value::Number(val2) => {
                                    self.execution_stack.push(Value::Bool(val2 < val1));
                                }
                                _ => panic!("Comparison of vairables of different types is not supported")
                            }
                        },
                        _ => {
                            panic!("Comparison of anything but numbers variables not supported")
                        }
                    }
                },
                OP_POP => {
                    self.advance();
                    self.execution_stack.pop();
                },
                OP_DECLARE => {
                    self.advance();
                    self.execution_stack.push(Value::Null);
                },
                OP_DECLARE_GLOBAL => {
                    self.advance();
                    let var_key = self.read_global();
                    let val = self.execution_stack.pop().unwrap();

                    self.globals.insert(var_key, val);
                }
                OP_PRINT => {
                    self.advance();
                    let val = self.execution_stack.pop().unwrap();
                    println!("{val}");
                },
                OP_SET => {
                    self.advance();
                    let val = self.execution_stack.pop().unwrap();
                    let var = self.peek().unwrap().clone();
                    self.advance();
                    self.execution_stack.push(val.clone());
                    self.execution_stack.set(var as i32, val);
                },
                OP_RETURN => {
                    self.advance();
                    let val = self.execution_stack.pop().unwrap();
                    self.execution_stack.set(-2, val);
                },
                OP_CALL => {
                    self.advance();
                    let Value::Function(ip, ar) = self.execution_stack.pop().unwrap() else {panic!("Wrong architecture");};
                    let mut args = vec![];
                    for _ in 0..ar {
                        args.push(self.execution_stack.pop().unwrap());
                    }
                    self.execution_stack.push(Value::Null);
                    self.execution_stack.push(Value::Number(self.ip as f32));

                    let old_offset = self.execution_stack.offset;
                    self.execution_stack.offset = self.execution_stack.real_len();

                    self.execution_stack.push(Value::Number(old_offset as f32));
                    args.into_iter().for_each(|arg| self.execution_stack.push(arg));

                    self.ip = ip;
                },
                OP_END_FUNCTION => {
                    self.execution_stack.reset_to(1usize);
                    let Value::Number(off) = self.execution_stack.pop().unwrap() else {panic!("Wrong architecture");};
                    self.execution_stack.offset = off as usize;
                    let Value::Number(ip) = self.execution_stack.pop().unwrap() else {panic!("Wrong architecture");};
                    self.ip = ip as usize;
                },
                OP_SET_GLOBAL => {
                    self.advance();
                    let val = self.execution_stack.pop().unwrap();
                    let var = self.read_global();
                    self.execution_stack.push(val.clone());
                    self.globals.insert(var, val);
                }
                _ => {
                    todo!();
                }
            }
        }
    }

    pub fn init(compiler: Compiler) -> Self {
        Self {
            compiler,
            execution_stack: Stack::init(),
            ip: 0usize,
            globals: HashMap::new()
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

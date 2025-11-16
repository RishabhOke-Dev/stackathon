use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{lexer::{Token, TokenPosition, TokenType}, types::{Keyword, Operation, Value}};

#[derive(Debug)]
pub struct Stack {
    data: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {data: Vec::new()}
    }

    fn push(&mut self, val: Value) {
        self.data.push(val);
    }

    fn pop(&mut self) -> Option<Value> {
        self.data.pop()
    }

}


#[derive(Debug)]
pub enum RuntimeError {
    OperatorInvalidValues(TokenPosition, char),
    KeywordInvalidValues(TokenPosition, Keyword),
}

impl RuntimeError {
    pub fn position(&self) -> TokenPosition {
        match self {
            RuntimeError::OperatorInvalidValues(pos, _) => *pos,
            RuntimeError::KeywordInvalidValues(pos, _) => *pos,
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OperatorInvalidValues(pos, op) =>
                write!(f,"Runtime Error({}:{}): Incorrect values provided for operator '{}'", pos.col, pos.row, op),
            Self::KeywordInvalidValues(pos, k) =>
                write!(f,"Runtime Error({}:{}): Incorrect values provided for keyword '{:?}'", pos.col, pos.row, k)
        }
    }
}

impl Error for RuntimeError {}

pub fn execute(tokens: &Vec<Token>, stack: &mut Stack, function_table: &HashMap<String, Vec<Token>>) -> Result<(), RuntimeError>{

    for token in tokens {
        match &token.kind {
            TokenType::Literal(lit) => match lit {
                Value::Function(func) => {
                    let function_definition = function_table.get(func).unwrap();

                    if function_definition.is_empty() {
                        stack.push(Value::Tag(func.clone()));
                    } else {
                        stack.push(Value::Block(function_definition.clone()));
                    }
                },
                _ => stack.push(lit.clone())
            },
            TokenType::Op(op) => {

                match op {
                    Operation::Add => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '+'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '+'))
                        };
                        stack.push(match val1 + val2 {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '+'))
                        });
                    },
                    Operation::Divide => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '/'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '/'))
                        };
                        stack.push(match val2 / val1 {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '/'))
                        });
                    },
                    Operation::Multiply => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '*'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '*'))
                        };
                        stack.push(match val1 * val2 {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '*'))
                        });
                    },
                    Operation::Subtract => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '-'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '-'))
                        };
                        stack.push(match val2 - val1 {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '-'))
                        });
                    },
                    Operation::Equal => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '='))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '='))
                        };
                        stack.push(Value::Boolean(val1 == val2));
                    },
                    Operation::NotEqual => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '!'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '!'))
                        };
                        stack.push(Value::Boolean(val1 != val2));
                    },
                    Operation::Not => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '!')),
                        };
                        stack.push(match !val1 {
                            Some(b) => b,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '!')),
                        });
                    },
                    Operation::Lesser => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '<'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '<'))
                        };
                        stack.push(Value::Boolean(val2 < val1));
                    },
                    Operation::LesserEqual => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '<'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '<'))
                        };
                        stack.push(Value::Boolean(val2 <= val1));
                    },
                    Operation::Greater => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '>'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '>'))
                        };
                        stack.push(Value::Boolean(val2 > val1));
                    },
                    Operation::GreaterEqual => {
                        let val1 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '>'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '>'))
                        };
                        stack.push(Value::Boolean(val2 >= val1));
                    },
                    Operation::And => {
                        let val1 = match stack.pop() {
                            Some(v) => match v {
                                Value::Boolean(b) => b,
                                _ => return Err(RuntimeError::OperatorInvalidValues(token.pos, '&')),
                            },
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '&'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => match v {
                                Value::Boolean(b) => b,
                                _ => return Err(RuntimeError::OperatorInvalidValues(token.pos, '&')),
                            },
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '&'))
                        };
                        stack.push(Value::Boolean(val1 && val2));
                    },
                    Operation::Or => {
                        let val1 = match stack.pop() {
                            Some(v) => match v {
                                Value::Boolean(b) => b,
                                _ => return Err(RuntimeError::OperatorInvalidValues(token.pos, '|')),
                            },
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '|'))
                        };
                        let val2 = match stack.pop() {
                            Some(v) => match v {
                                Value::Boolean(b) => b,
                                _ => return Err(RuntimeError::OperatorInvalidValues(token.pos, '|')),
                            },
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '|'))
                        };
                        stack.push(Value::Boolean(val1 || val2));
                    },
                    Operation::Run => {
                        let val1 = match stack.pop() {
                            Some(v) => match v {
                                Value::Block(b) => b,
                                _ => return Err(RuntimeError::OperatorInvalidValues(token.pos, '$')),
                            },
                            None => return Err(RuntimeError::OperatorInvalidValues(token.pos, '$'))
                        };
                        execute(&val1, stack, function_table)?;
                    },
                }
            },
            TokenType::Keyword(keyword) => {
                match keyword {
                    Keyword::PRINT => print!("{}", stack.pop().unwrap_or(Value::String("".to_string()))),
                    Keyword::EXIT => return Ok(()),
                    Keyword::LOOP => {
                        let function = match stack.pop() {
                            Some(v) => match v {
                                Value::Block(b) => b,
                                _ => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::LOOP)),
                            },
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::LOOP))
                        };

                        loop {
                            let condition = match stack.pop() {
                                Some(v) => match v {
                                    Value::Boolean(b) => b,
                                    _ => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::LOOP)),
                                },
                                None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::LOOP))
                            };
                            if !condition {
                                break;
                            }
                            execute(&function, stack, function_table)?;
                        }
                        
                        

                    },
                    Keyword::DUPLICATE => {
                        let val = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::DUPLICATE))
                        };
                        stack.push(val.clone());
                        stack.push(val);
                    },
                    Keyword::DROP => {
                        stack.pop();
                    },
                    Keyword::SWAP => {
                        if stack.data.len() < 2 {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::SWAP));
                        }
                        let a = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::SWAP)),
                        };
                        let b = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::SWAP)),
                        };
                        stack.push(b);
                        stack.push(a);
                    },
                    Keyword::DEPTH => {
                        stack.push(Value::Integer(stack.data.len() as i32));
                    },
                    Keyword::ROT => {
                        let len = stack.data.len();
                        if len < 3 {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::ROT));
                        }
                        stack.data.swap(len - 1, len - 2);
                        stack.data.swap(len - 3, len - 1);
                    },
                    Keyword::NROT => {
                        let len = stack.data.len();
                        if len < 3 {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::NROT));
                        }
                        stack.data.swap(len - 3, len - 1);
                        stack.data.swap(len - 2, len - 1);
                    },
                    Keyword::OVER => {
                        let len = stack.data.len();
                        if len < 2 {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::OVER));
                        }
                        let val = stack.data[len - 2].clone();
                        stack.push(val);
                    },
                    Keyword::TUCK =>  {
                        let len = stack.data.len();
                        if len < 2 {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::TUCK));
                        }
                        let val = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::TUCK))
                        };
                        stack.data.insert(len - 2, val.clone());
                        stack.push(val);
                    },
                    Keyword::PICK => {
                        let index = match stack.pop() {
                            Some(v) => match v {
                                Value::Integer(i) => i-1,
                                _ => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::PICK))
                            },
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::PICK))
                        };
                        if index < 0 {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::ROLL));
                        }
                        let index = index as usize;
                        if stack.data.len() <= index {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::PICK));
                        }
                        let val = stack.data[index].clone();
                        stack.push(val);
                    },
                    Keyword::ROLL => {
                        let index = match stack.pop() {
                            Some(v) => match v {
                                Value::Integer(i) => i-1,
                                _ => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::ROLL))
                            },
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::ROLL))
                        };
                        if index < 0 {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::ROLL));
                        }
                        let index = index as usize;
                        if stack.data.len() <= index {
                            return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::ROLL));
                        }
                        let val = stack.data.remove(index);
                        stack.push(val);
                    },
                    Keyword::CLEAR => {
                        stack.data.clear();
                    },
                    Keyword::GATE => {
                        let true_func = match stack.pop() {
                            Some(v) => match v {
                                Value::Block(b) => b,
                                _ => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::GATE)),
                            },
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::GATE))
                        };
                        let mut false_func = Vec::new();
                        let cond;

                        match stack.pop() {
                            Some(v) => match v {
                                Value::Block(b) => {
                                    false_func = b;
                                    match stack.pop() {
                                        Some(v) => match v {
                                            Value::Boolean(b) => cond = b,
                                            _ => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::GATE)),
                                        },
                                        None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::GATE))
                                    }
                                },
                                Value::Boolean(boolean) => cond = boolean,
                                _ => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::GATE)),
                            },
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::GATE))
                        }
                        
                        if cond {
                            execute(&true_func, stack, function_table)?;
                        } else {
                            execute(&false_func, stack, function_table)?;
                        }
                    },
                    Keyword::TYPE => {
                        let val = match stack.pop() {
                            Some(v) => v,
                            None => return Err(RuntimeError::KeywordInvalidValues(token.pos, Keyword::TYPE)),
                        };
                        stack.push(Value::Tag(
                            match val {
                                Value::Block(_) => "block".to_string(),
                                Value::Integer(_) => "int".to_string(),
                                Value::Float(_) => "float".to_string(),
                                Value::String(_) => "string".to_string(),
                                Value::Boolean(_) => "bool".to_string(),
                                Value::Tag(t) => t,
                                Value::Function(_) => "function".to_string(), //This one should be impossible (for now) because functions turn into blocks when pushed
                            }
                        ));
                    },
                    _ => ()//unused keywords,
                }
            },
        }
        //println!("{:#?}", stack);
    }

    Ok(())
}
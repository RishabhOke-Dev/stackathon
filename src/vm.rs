use std::{error::Error, fmt::Display};

use crate::{lexer::{Token, TokenPosition, TokenType}, types::{Keyword, Operation, Value}};

#[derive(Debug)]
pub struct Stack {
    data: Vec<Value>,
}

impl Stack {
    fn new() -> Self {
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

pub fn execute(tokens: &Vec<Token>, stack: Option<&mut Stack>) -> Result<(), RuntimeError>{

    let stack = match stack {
        Some(s) => s,
        None => &mut Stack::new(),
    };

    for token in tokens {
        match &token.kind {
            TokenType::Literal(lit) => stack.push(lit.clone()),
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
                        stack.push(match val1 / val2 {
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
                        stack.push(match val1 - val2 {
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
                        stack.push(Value::Boolean(val1 < val2));
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
                        stack.push(Value::Boolean(val1 <= val2));
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
                        stack.push(Value::Boolean(val1 > val2));
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
                        stack.push(Value::Boolean(val1 >= val2));
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
                        execute(&val1, Some(stack))?;
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
                            execute(&function, Some(stack))?;
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
                            execute(&true_func, Some(stack))?;
                        } else {
                            execute(&false_func, Some(stack))?;
                        }
                    }
                    _ => ()//unused keywords,
                }
            },
        }
        //println!("{:#?}", stack);
    }

    Ok(())
}
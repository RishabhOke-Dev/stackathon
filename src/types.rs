
use std::{fmt::Display, ops::{Add, Div, Mul, Not, Sub}};

use crate::{lexer::Token, serial::{ByteSized, SerializationError}};

#[derive(Debug)]
#[derive(Clone)]
pub enum Keyword {
    PRINT,
    TRUE, //True and false don't do anything but push respective boolean.
    FALSE,
    EXIT,
    LOOP, //Control flow! used like <cond> <code> loop
    GATE,
    DUPLICATE,//All the stack manip keywords
    DROP,
    SWAP,
    DEPTH,
    ROT,
    NROT,
    OVER,
    TUCK,
    PICK,
    ROLL,
    CLEAR,
    TYPE, //gets the type and pushes it onto the stack. The type of the value it pushes is "Tag"
    USE, //Library invokation
    INPUT, //Gets user input
    STRLEN, //Gets length of string
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Value {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String),
    Block(Vec<Token>),
    Function(String),
    Tag(String), //Is both a manual tag eg. @list or the result of a type eg. 2 type
}


impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(int) => write!(f, "{}", int),
            Value::Float(float) => write!(f, "{}", float),
            Value::Boolean(boolean) => write!(f, "{}", if *boolean {"true"} else {"false"}),
            Value::String(string) => write!(f, "{}", string),
            Value::Block(tok) => write!(f, "{:?}", tok) /*TODO change this to something that makes sense*/,
            Value::Function(fun) => write!(f, "{}", fun),
            Value::Tag(str) => write!(f, "{}", str),
        }
    }
}

impl Add for Value {
    type Output = Option<Value>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(v1), Value::Float(v2)) => Some(Value::Float(v1 + v2)),
            (Value::Float(v1), Value::Integer(v2)) => Some(Value::Float(v1 + v2 as f32)),
            (Value::Integer(v1), Value::Float(v2)) => Some(Value::Float(v1 as f32 + v2)),
            (Value::Integer(v1), Value::Integer(v2)) => Some(Value::Integer(v1 + v2)),
            (Value::String(v1), Value::String(v2)) => Some(Value::String(v1 + &v2)),
            _ => None   
        }
    }
}

impl Sub for Value {
    type Output = Option<Value>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(v1), Value::Float(v2)) => Some(Value::Float(v1 - v2)),
            (Value::Float(v1), Value::Integer(v2)) => Some(Value::Float(v1 - v2 as f32)),
            (Value::Integer(v1), Value::Float(v2)) => Some(Value::Float(v1 as f32 - v2)),
            (Value::Integer(v1), Value::Integer(v2)) => Some(Value::Integer(v1 - v2)),
            _ => None   
        }
    }
}

impl Mul for Value {
    type Output = Option<Value>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(v1), Value::Float(v2)) => Some(Value::Float(v1 * v2)),
            (Value::Float(v1), Value::Integer(v2)) => Some(Value::Float(v1 * v2 as f32)),
            (Value::Integer(v1), Value::Float(v2)) => Some(Value::Float(v1 as f32 * v2)),
            (Value::Integer(v1), Value::Integer(v2)) => Some(Value::Integer(v1 * v2)),
            (Value::String(v1), Value::Integer(v2)) => Some(Value::String(v1.repeat(v2 as usize))), //Won't work on <32 bit address size cpu
            _ => None   
        }
    }
}

impl Div for Value {
    type Output = Option<Value>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(v1), Value::Float(v2)) => Some(Value::Float(v1 / v2)),
            (Value::Float(v1), Value::Integer(v2)) => Some(Value::Float(v1 / v2 as f32)),
            (Value::Integer(v1), Value::Float(v2)) => Some(Value::Float(v1 as f32 / v2)),
            (Value::Integer(v1), Value::Integer(v2)) => Some(Value::Float(v1 as f32 / v2 as f32)),
            (Value::String(v1), Value::Integer(v2)) => match v1.chars().nth((v2 as usize) - 1) {Some(s) => Some(Value::String(s.to_string())), None => None}
            _ => None   
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Float(v1), Value::Float(v2)) => v1 == v2,
            (Value::Float(v1), Value::Integer(v2)) => *v1 == *v2 as f32,
            (Value::Integer(v1), Value::Float(v2)) => *v1 as f32 == *v2,
            (Value::Integer(v1), Value::Integer(v2)) => v1 == v2,
            (Value::Boolean(v1), Value::Boolean(v2)) => v1 == v2,
            (Value::String(v1), Value::String(v2)) => v1 == v2,
            (Value::Function(f), Value::Function(f2)) => f == f2,
            (Value::Tag(t), Value::Tag(t2)) => t == t2,
            _ => false,
        }
    }
}

impl Not for Value {
    type Output = Option<Value>;
    fn not(self) -> Self::Output {
        match self {
            Value::Boolean(b) => Some(Value::Boolean(!b)),
            _ => None,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Float(v1), Value::Float(v2)) => v1.partial_cmp(v2),
            (Value::Float(v1), Value::Integer(v2)) => v1.partial_cmp(&(*v2 as f32)),
            (Value::Integer(v1), Value::Float(v2)) => (*v1 as f32).partial_cmp(v2),
            (Value::Integer(v1), Value::Integer(v2)) => v1.partial_cmp(v2),
            (Value::Boolean(v1), Value::Boolean(v2)) => v1.partial_cmp(v2),
            (Value::String(v1), Value::String(v2)) => v1.partial_cmp(v2),
            _ => None,
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Greater,
    Lesser,
    GreaterEqual,
    LesserEqual,
    And,
    Or,
    Not,
    Run,
}

impl ByteSized for Keyword {
    fn to_bytes(&self) -> Vec<u8> {

        let binary: u8 = match self {
            Keyword::PRINT => 0x01,
            Keyword::TRUE => 0x02,
            Keyword::FALSE => 0x03,
            Keyword::EXIT => 0x04,
            Keyword::LOOP => 0x05,
            Keyword::GATE => 0x06,
            Keyword::DUPLICATE => 0x07,
            Keyword::DROP => 0x08,
            Keyword::SWAP => 0x09,
            Keyword::DEPTH => 0x0A,
            Keyword::ROT => 0x0B,
            Keyword::NROT => 0x0C,
            Keyword::OVER => 0x0D,
            Keyword::TUCK => 0x0E,
            Keyword::PICK => 0x0F,
            Keyword::ROLL => 0x10,
            Keyword::CLEAR => 0x11,
            Keyword::TYPE => 0x12,
            Keyword::USE => 0x13,
            Keyword::INPUT => 0x14,
            Keyword::STRLEN => 0x15,
        };
        vec![binary]
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
    where
        Self: Sized
    {
        if bytes.is_empty() {
            return Err(SerializationError::EndOfFile);
        }
        let tag = bytes[0];

        let keyword = match tag {
            0x01 => Keyword::PRINT,
            0x02 => Keyword::TRUE,
            0x03 => Keyword::FALSE,
            0x04 => Keyword::EXIT,
            0x05 => Keyword::LOOP,
            0x06 => Keyword::GATE,
            0x07 => Keyword::DUPLICATE,
            0x08 => Keyword::DROP,
            0x09 => Keyword::SWAP,
            0x0A => Keyword::DEPTH,
            0x0B => Keyword::ROT,
            0x0C => Keyword::NROT,
            0x0D => Keyword::OVER,
            0x0E => Keyword::TUCK,
            0x0F => Keyword::PICK,
            0x10 => Keyword::ROLL,
            0x11 => Keyword::CLEAR,
            0x12 => Keyword::TYPE,
            0x13 => Keyword::USE,
            0x14 => Keyword::INPUT,
            0x15 => Keyword::STRLEN,
            _ => return Err(SerializationError::InvalidTagByte(tag))
        };

        Ok((keyword, 1))
    }
}

impl ByteSized for Operation {
    fn to_bytes(&self) -> Vec<u8> {
        let binary: u8 = match self {
            Operation::Add => 0x01,
            Operation::Subtract => 0x02,
            Operation::Multiply => 0x03,
            Operation::Divide => 0x04,
            Operation::Equal => 0x05,
            Operation::NotEqual => 0x06,
            Operation::Greater => 0x07,
            Operation::Lesser => 0x08,
            Operation::GreaterEqual => 0x09,
            Operation::LesserEqual => 0x0A,
            Operation::And => 0x0B,
            Operation::Or => 0x0C,
            Operation::Not => 0x0D,
            Operation::Run => 0x0E,
        };
        vec![binary]
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), crate::serial::SerializationError>
    where
        Self: Sized
    {
        if bytes.is_empty() {
            return Err(SerializationError::EndOfFile);
        }
        let tag = bytes[0];

        let operation = match tag {
            0x01 => Operation::Add,
            0x02 => Operation::Subtract,
            0x03 => Operation::Multiply,
            0x04 => Operation::Divide,
            0x05 => Operation::Equal,
            0x06 => Operation::NotEqual,
            0x07 => Operation::Greater,
            0x08 => Operation::Lesser,
            0x09 => Operation::GreaterEqual,
            0x0A => Operation::LesserEqual,
            0x0B => Operation::And,
            0x0C => Operation::Or,
            0x0D => Operation::Not,
            0x0E => Operation::Run,
            _ => return Err(SerializationError::InvalidTagByte(tag))
        };
        Ok((operation, 1))
    }
}

impl ByteSized for Value {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Integer(i) => {
                let mut bytes = vec![0x01];

                bytes.extend_from_slice(&i.to_be_bytes());
                bytes
            },
            Value::Float(f) => {
                let mut bytes = vec![0x02];

                bytes.extend_from_slice(&f.to_be_bytes());
                bytes
            },
            Value::Boolean(b) => {
                let mut bytes = vec![0x03];

                bytes.push(match b {
                    true => 0x01,
                    false => 0x00,
                });
                bytes
            },
            Value::String(s) => {
                let mut bytes = vec![0x04];
                
                let s_bytes = s.as_bytes();

                let length = s_bytes.len() as u32;
                
                bytes.extend_from_slice(&length.to_be_bytes());

                bytes.extend_from_slice(s_bytes);
                bytes
            },
            Value::Block(t) => {
                let mut bytes = vec![0x05]; //Tag byte

                let length = t.len() as u32; //Get the length

                bytes.extend_from_slice(&length.to_be_bytes()); //Push the length

                for token in t {
                    bytes.append(&mut token.to_bytes()); //Push each token
                }

                bytes 
            },
            Value::Function(s) => {
                let mut bytes = vec![0x06];
                
                let s_bytes = s.as_bytes();

                let length = s_bytes.len() as u32;
                
                bytes.extend_from_slice(&length.to_be_bytes());

                bytes.extend_from_slice(s_bytes);
                bytes
            },
            Value::Tag(s) => {
                let mut bytes = vec![0x07];
                
                let s_bytes = s.as_bytes();

                let length = s_bytes.len() as u32;
                
                bytes.extend_from_slice(&length.to_be_bytes());

                bytes.extend_from_slice(s_bytes);
                bytes
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
        where
            Self: Sized
    {
        if bytes.is_empty() {
            return Err(SerializationError::EndOfFile);
        }
        let tag = bytes[0];
        match tag {
            0x01 => {
                if bytes.len() < 5 {
                    return Err(SerializationError::EndOfFile);
                }
                let payload: [u8; 4]  = bytes[1..5].try_into().unwrap(); //Never panics because we checked the length
                Ok((Value::Integer(i32::from_be_bytes(payload)), 5))
            },
            0x02 => {
                if bytes.len() < 5 {
                    return Err(SerializationError::EndOfFile);
                }
                let payload: [u8; 4]  = bytes[1..5].try_into().unwrap(); //Never panics because we checked the length
                Ok((Value::Float(f32::from_be_bytes(payload)), 5))
            },
            0x03 => {
                if bytes.len() < 2 {
                    return Err(SerializationError::EndOfFile);
                }
                let payload = match bytes[1] {
                    0x01 => true,
                    0x00 => false,
                    _ => return Err(SerializationError::InvalidTagByte(bytes[1]))
                };
                Ok((Value::Boolean(payload), 2))
            },
            0x04 => {
                if bytes.len() < 5 { //Check we have enough bytes for the length of the string
                    return Err(SerializationError::EndOfFile);
                }
                let len = u32::from_be_bytes(bytes[1..5].try_into().unwrap());//Unwrap is okay because we checked the length
                if bytes.len() < 5 + len as usize { //Check if we have enough bytes for the data of the string
                    return Err(SerializationError::EndOfFile);
                }
                let payload = &bytes[5..5+len as usize];
                let string = match String::from_utf8(payload.to_vec()) {
                    Ok(s) => s,
                    Err(e) => return Err(SerializationError::InvalidUTF8Encoding(e))
                };

                return Ok((Value::String(string),5+len as usize));
            },
            0x05 => {
                if bytes.len() < 5 { //Check if we have enough bytes for the length of the block
                    return Err(SerializationError::EndOfFile);
                }
                let len = u32::from_be_bytes(bytes[1..5].try_into().unwrap()); //Get length and unwrap is okay because we checked length
                if bytes.len() < 5 + len as usize { //Check if we enough bytes for the tokens
                    return Err(SerializationError::EndOfFile);
                }
                let mut tokens = Vec::new();
                let mut offset = 5;
                for _ in 0..len {
                    let (token, new_offset) = Token::from_bytes(&bytes[offset..])?;
                    offset += new_offset;
                    tokens.push(token);
                }
                return Ok((Value::Block(tokens) ,offset))
            },
            0x06 => {
                if bytes.len() < 5 { //Check we have enough bytes for the length of the string
                    return Err(SerializationError::EndOfFile);
                }
                let len = u32::from_be_bytes(bytes[1..5].try_into().unwrap());//Unwrap is okay because we checked the length
                if bytes.len() < 5 + len as usize { //Check if we have enough bytes for the data of the string
                    return Err(SerializationError::EndOfFile);
                }
                let payload = &bytes[5..5+len as usize];
                let string = match String::from_utf8(payload.to_vec()) {
                    Ok(s) => s,
                    Err(e) => return Err(SerializationError::InvalidUTF8Encoding(e))
                };

                return Ok((Value::Function(string),5+len as usize));
            },
            0x07 => {
                if bytes.len() < 5 { //Check we have enough bytes for the length of the string
                    return Err(SerializationError::EndOfFile);
                }
                let len = u32::from_be_bytes(bytes[1..5].try_into().unwrap());//Unwrap is okay because we checked the length
                if bytes.len() < 5 + len as usize { //Check if we have enough bytes for the data of the string
                    return Err(SerializationError::EndOfFile);
                }
                let payload = &bytes[5..5+len as usize];
                let string = match String::from_utf8(payload.to_vec()) {
                    Ok(s) => s,
                    Err(e) => return Err(SerializationError::InvalidUTF8Encoding(e))
                };

                return Ok((Value::Tag(string),5+len as usize));
            },
            _ => return Err(SerializationError::InvalidTagByte(tag))
        }
    }
}
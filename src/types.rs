
use std::{fmt::Display, ops::{Add, Div, Mul, Not, Sub}};

use crate::lexer::Token;

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

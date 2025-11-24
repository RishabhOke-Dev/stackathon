use std::{collections::HashMap, error::Error, fmt, iter::Peekable, str::Chars, sync::OnceLock};

use crate::{serial::{ByteSized, SerializationError}, types::{Keyword, Operation, Value}};

static KEYWORDS: OnceLock<HashMap<&'static str, Keyword>> = OnceLock::new(); 

#[derive(Debug)]
pub struct TokenPosition {
    pub row: usize,
    pub col: usize,
}

impl ByteSized for TokenPosition {
    fn to_bytes(&self) -> Vec<u8> {
        let mut byte = (self.row as u32).to_be_bytes().to_vec();

        byte.extend_from_slice(&(self.col as u32).to_be_bytes());

        byte
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
        where
            Self: Sized
    {
        if bytes.len() < 8 {
            return Err(SerializationError::EndOfFile);
        }

        let row = u32::from_be_bytes(bytes[0..4].try_into().unwrap()) as usize; //Unwrap is fine because we checked size
        let col = u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize; //Same here

        return Ok((TokenPosition {row, col}, 8));
    }
}

impl Clone for TokenPosition {
    fn clone(&self) -> Self {
        TokenPosition { row: self.row, col: self.col }
    }
}

impl Copy for TokenPosition {}

#[derive(Debug)]
#[derive(Clone)]
pub enum TokenType {
    Literal(Value),
    Op(Operation),
    Keyword(Keyword),
}

impl ByteSized for TokenType {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            TokenType::Literal(v) => {
                let mut bytes = vec![0x01];
                bytes.extend_from_slice(&v.to_bytes());
                bytes
            },
            TokenType::Op(o) => {
                let mut bytes = vec![0x02];
                bytes.extend_from_slice(&o.to_bytes());
                bytes
            },
            TokenType::Keyword(k) => {
                let mut bytes = vec![0x03];
                bytes.extend_from_slice(&k.to_bytes());
                bytes
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), crate::serial::SerializationError>
        where
            Self: Sized
    {
        if bytes.len() < 2 {
            return Err(SerializationError::EndOfFile);
        }
        match bytes[0] {
            0x01 => {
                let (value, bytes_read) = Value::from_bytes(&bytes[1..])?;
                Ok((TokenType::Literal(value), bytes_read + 1))
            },
            0x02 => {
                let (value, bytes_read) = Operation::from_bytes(&bytes[1..])?;
                Ok((TokenType::Op(value), bytes_read + 1))
            },
            0x03 => {
                let (value, bytes_read) = Keyword::from_bytes(&bytes[1..])?;
                Ok((TokenType::Keyword(value), bytes_read + 1))
            },
            _ => return Err(SerializationError::InvalidTagByte(bytes[0]))
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub pos: TokenPosition,
}

impl Token {
    fn new(kind: TokenType, row: usize, col: usize) -> Self {
        Token {
            kind,
            pos: TokenPosition {
                row,
                col,
            }
        }
    }
}

impl ByteSized for Token {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.kind.to_bytes();
        bytes.append(&mut self.pos.to_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
        where
            Self: Sized
    {
        if bytes.is_empty() {
            return Err(SerializationError::EndOfFile);
        }
        let (kind, type_bytes) = TokenType::from_bytes(bytes)?;
        let (pos, pos_bytes) = TokenPosition::from_bytes(&bytes[type_bytes..])?;
        return Ok((Token {kind, pos}, type_bytes + pos_bytes));
    }
}


#[derive(Debug)]
pub enum TokenizerError {
    InvalidNumberFormat(TokenPosition),
    UnexpectedSymbol(TokenPosition, char),
    UnknownIdentifier(TokenPosition, String),
    BlockHadNoEnd(TokenPosition),
    StringHadNoEnd(TokenPosition),
    FunctionHasMultipleDefinitions(TokenPosition, String),
}

impl TokenizerError {
    pub fn position(&self) -> TokenPosition {
        match self {
            TokenizerError::InvalidNumberFormat(pos) => *pos,
            TokenizerError::UnexpectedSymbol(pos, _) => *pos,
            TokenizerError::UnknownIdentifier(pos, _) => *pos,
            TokenizerError::BlockHadNoEnd(pos) => *pos,
            TokenizerError::StringHadNoEnd(pos) => *pos,
            TokenizerError::FunctionHasMultipleDefinitions(pos, _) => *pos,
        }
    }
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumberFormat(pos) =>
                write!(f, "Syntax error({}:{}): Invalid number format", pos.row, pos.col),
            Self::UnexpectedSymbol(pos,c) =>
                write!(f, "Syntax error({}:{}): Unexpected character '{}'", pos.row, pos.col, c),
            Self::UnknownIdentifier(pos, s) =>
                write!(f, "Syntax error({}:{}): Unknown identifier '{}'", pos.row, pos.col, s),
            Self::BlockHadNoEnd(pos) => 
                write!(f, "Syntax error({}:{}): Block has no matching brace", pos.row, pos.col),
            Self::StringHadNoEnd(pos) =>
                write!(f, "Syntax error({}:{}): String has no end.", pos.row, pos.col),
            Self::FunctionHasMultipleDefinitions(pos, func) =>
                write!(f, "Syntax error({}:{}): Function '{}' has multiple definitions.", pos.row, pos.col, func),
        }
    }
}

impl Error for TokenizerError {}


pub fn tokenize(code: &str, starting_position: Option<TokenPosition>, functions: &mut HashMap<String, Vec<Token>>) -> Result<Vec<Token>, TokenizerError> {
    
    let mut tokens = Vec::new();
    
    let mut code = code.chars().peekable();

    let mut position = match starting_position {None => TokenPosition {row: 1, col: 1}, Some(p) => p};
    while let Some(character) = code.next() {
        

        //handles comments
        if character == ';' {
            while let Some(c) = code.next() {
                position.col += 1;
                if c == ';' {
                    break;
                }
            }
        }


        let is_neg_num = character == '-' && code.peek().map_or(false, |&next_char| {
            next_char.is_ascii_digit() || next_char == '.'
        });

        //handles numbers
        if character.is_ascii_digit() || character == '.' || is_neg_num{
            let start_pos = position;

            let mut number_string = String::from(character);
            let mut saw_dot = character == '.';


            position.col += 1;

            while let Some(&next_character) = code.peek() {
                if next_character.is_ascii_digit() {
                    number_string.push(code.next().unwrap());
                    position.col += 1;
                } else if next_character == '.' {

                    if saw_dot {
                        code.next();
                        return Err(TokenizerError::InvalidNumberFormat(position));
                    }

                    saw_dot = true;
                    number_string.push(code.next().unwrap());
                    position.col += 1;
                } else if next_character.is_whitespace() {
                    break;
                } else {

                    code.next();
                    return Err(TokenizerError::InvalidNumberFormat(position));
                }
                

            }
            let value = if saw_dot {
                match number_string.parse::<f32>() {
                    Ok(i) => Value::Float(i),
                    //Never happens unless f32 is too small
                    Err(_) => return Err(TokenizerError::InvalidNumberFormat(position)),
                }
            } else {
                match number_string.parse::<i32>() {
                    Ok(i) => Value::Integer(i),
                    //Never happens unless i32 is too small
                    Err(_) => return Err(TokenizerError::InvalidNumberFormat(position)),
                }
            };
            

            tokens.push(Token::new(TokenType::Literal(value), start_pos.row, start_pos.col));
            continue;
        }
        //handles strings
        if character == '"' {
            let start_pos = position;
            let mut string = String::new();
            loop {
                let next_character = match code.peek() {
                    Some(&c) => c,
                    None => return Err(TokenizerError::StringHadNoEnd(start_pos))
                };
                if next_character == '"' {
                    
                    position.col += 1;
                    code.next();
                    if let Some(&next_character) = code.peek() && !next_character.is_whitespace() {
                        position.col += 1;
                        return Err(TokenizerError::UnexpectedSymbol(position, next_character))
                    }
                    tokens.push(Token::new(TokenType::Literal(Value::String(string)), start_pos.row, start_pos.col));
                    break;
                }
                if next_character == '\\' {
                   
                    code.next();
                    position.col += 1;
                    match code.next() {
                        Some('n') => string.push('\n'),
                        Some('t') => string.push('\t'),
                        Some('\\') => string.push('\\'),
                        Some('"') => string.push('\"'),
                        Some('r') => string.push('\r'),

                        Some(other) => {
                            string.push('\\');
                            string.push(other);
                        }

                        None => {
                            string.push('\\');
                        }
                    }
                    
                    position.col += 1;
                    continue;
                }

                string.push(next_character);
                position.col += 1;
                code.next();
            }
            
        }
        //handles blocks
        if character == '{' {
            let p = position;
            let token = handle_block(&mut position, &mut code, functions)?;
            tokens.push(Token::new(TokenType::Literal(Value::Block(token)), p.row, p.col));
        }

        if character == '}' {
            return Err(TokenizerError::BlockHadNoEnd(position))
        }
        //handles operators
        if character == '+' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::Add),position.row, position.col));
            
        }
        if character == '-' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::Subtract),position.row, position.col));
        }
        if character == '/' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::Divide),position.row, position.col));
        }
        if character == '*' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::Multiply),position.row, position.col));
        }
        if character == '=' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::Equal), position.row, position.col));
        }
        if character == '!' {
            let mut next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if next_character == '=' {
                code.next();
                position.col += 1;
                next_character = code.peek().map(|&c| c).unwrap_or(' ');
                if !next_character.is_whitespace() {
                    position.col += 1;
                    return Err(TokenizerError::UnexpectedSymbol(position, next_character));
                }
                tokens.push(Token::new(TokenType::Op(Operation::NotEqual), position.row, position.col));
                continue;
            }
            tokens.push(Token::new(TokenType::Op(Operation::Not), position.row, position.col));
        }
        if character == '<' {
            let mut next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if next_character == '=' {
                code.next();
                position.col += 1;
                next_character = code.peek().map(|&c| c).unwrap_or(' ');
                if !next_character.is_whitespace() {
                    position.col += 1;
                    return Err(TokenizerError::UnexpectedSymbol(position, next_character));
                }
                tokens.push(Token::new(TokenType::Op(Operation::LesserEqual), position.row, position.col));
                continue;
            }
            tokens.push(Token::new(TokenType::Op(Operation::Lesser), position.row, position.col));
        }
        if character == '>' {
            let mut next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if next_character == '=' {
                code.next();
                position.col += 1;
                next_character = code.peek().map(|&c| c).unwrap_or(' ');
                if !next_character.is_whitespace() {
                    position.col += 1;
                    return Err(TokenizerError::UnexpectedSymbol(position, next_character));
                }
                tokens.push(Token::new(TokenType::Op(Operation::GreaterEqual), position.row, position.col));
                continue;
            }
            tokens.push(Token::new(TokenType::Op(Operation::Greater), position.row, position.col));
        }
        if character == '&' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::And),position.row, position.col));
        }
        if character == '|' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::Or),position.row, position.col));
        }
        if character == '$' {
            let next_character = code.peek().map(|&c| c).unwrap_or(' ');
            if !next_character.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(position, next_character));
            }
            tokens.push(Token::new(TokenType::Op(Operation::Run),position.row, position.col));
        }

        //handles function def
        if character == '@' {
            position.col += 1;
            let start_pos = position;
            let mut function_name = String::new();
            loop {
                let next_char = match code.next() {
                    Some(c) => c,
                    None => break,
                };

                if next_char.is_whitespace() {
                    break;
                }

                if !(next_char.is_ascii_alphanumeric() || next_char == '_') {   
                    return Err(TokenizerError::UnexpectedSymbol(position, next_char))
                }
                    
                function_name.push(next_char);
                position.col += 1;
            }
            if functions.contains_key(&function_name) {
                return Err(TokenizerError::FunctionHasMultipleDefinitions(start_pos, function_name));
            }
            position.col += 1;
            let c = code.next().unwrap_or(' ');
            if c != '{' {
               functions.insert(function_name, Vec::new());
            } else {
                functions.insert(function_name.clone(), Vec::new());
                let definition = handle_block(&mut position, &mut code, functions)?;
                functions.insert(function_name, definition);
            }
        }

        //handles keywords and identifiers
        if character.is_ascii_alphabetic() || character == '_' {
            let starting_position = position;
            let mut ident = String::from(character);
            while let Some(&next_character) = code.peek() {
                if next_character.is_whitespace() {
                    break;
                }
                if !next_character.is_ascii_alphanumeric() && next_character != '_' {
                    return Err(TokenizerError::UnexpectedSymbol(position, next_character));
                }
                ident.push(code.next().unwrap());
                position.col += 1;
            }
            let keyword_map = get_keywords();
            //We don't jump directly to a token type because of 'true' and 'false' keywords.
            let keyword =  match keyword_map.get(ident.as_str()) {
                Some(key) => key.clone(),
                None => {
                    if functions.contains_key(&ident) {
                        tokens.push(Token::new(TokenType::Literal(Value::Function(ident)),starting_position.row, starting_position.col));
                        continue;
                    } else {
                        return Err(TokenizerError::UnknownIdentifier(starting_position, ident));
                    }
                }
            };
            if let Keyword::TRUE = keyword {
                tokens.push(Token::new(TokenType::Literal(Value::Boolean(true)), starting_position.row, starting_position.col));
                
                continue;
            }
            if let Keyword::FALSE = keyword {
                tokens.push(Token::new(TokenType::Literal(Value::Boolean(false)), starting_position.row, starting_position.col));
                
                continue;
            }
            if let Keyword::EXIT = keyword {
                return Ok(tokens)
            }
            tokens.push(Token::new(TokenType::Keyword(keyword), starting_position.row, starting_position.col));
        }


        if character == '\n' {
            position.row += 1;
            position.col = 1;
        } else {
            position.col += 1;
        }
        
    }


    Ok(tokens)
}

fn handle_block(position: &mut TokenPosition, code: &mut Peekable<Chars>, functions: &mut HashMap<String, Vec<Token>>) -> Result<Vec<Token>, TokenizerError> {
    if let Some(c) = code.next() && !c.is_whitespace() {
        position.col += 1;
        return Err(TokenizerError::UnexpectedSymbol(*position, c))
    }
    position.col += 1;
    let starting_position = *position;
    let mut inner_code = String::new();
    let mut block_balancer = 1;
    loop { 
        let character = match code.next() {
            Some(c) => c,
            None => {return Err(TokenizerError::BlockHadNoEnd(*position));}
        };

        if character == '}' {
            position.col += 1;
            let c = code.next().unwrap_or(' ');
            if !c.is_whitespace() {
                position.col += 1;
                return Err(TokenizerError::UnexpectedSymbol(*position, c))
            }
            block_balancer -= 1;
            if block_balancer == 0 {
                inner_code.push('\0');
                position.col += 1;
                break;
            } else if block_balancer < 0 {
                return Err(TokenizerError::BlockHadNoEnd(*position))
            }
        }

        if character == '{' {
            block_balancer += 1;
        }

        if character == '\\' {
            if let Some(c) = code.next() {
                if c == '{' || c == '}' {
                    inner_code.push(c);
                    position.col += 2;
                    continue;
                } else {
                    inner_code.push('\\');
                    inner_code.push(c);
                    position.col += 2;
                    continue;
                }
            } 
        }

        inner_code.push(character);
        if character == '\n' {
            position.row += 1;
            position.col = 0;
        } else {
            position.col += 1;
        }
    }
    return Ok(tokenize(&inner_code, Some(starting_position), functions)?)
}


fn get_keywords() -> &'static HashMap<&'static str, Keyword> {
    KEYWORDS.get_or_init(|| {

        let mut map = HashMap::new();
        map.insert("print", Keyword::PRINT);
        map.insert("true", Keyword::TRUE);
        map.insert("false", Keyword::FALSE);
        map.insert("exit", Keyword::EXIT);
        map.insert("loop", Keyword::LOOP);
        map.insert("dup", Keyword::DUPLICATE);
        map.insert("gate", Keyword::GATE);
        map.insert("drop", Keyword::DROP);
        map.insert("type", Keyword::TYPE);
        map.insert("swap", Keyword::SWAP);
        map.insert("depth", Keyword::DEPTH);
        map.insert("rot", Keyword::ROT);
        map.insert("nrot", Keyword::NROT);
        map.insert("over", Keyword::OVER);
        map.insert("tuck", Keyword::TUCK);
        map.insert("pick", Keyword::PICK);
        map.insert("roll", Keyword::ROLL);
        map.insert("clear", Keyword::CLEAR);
        map
    })
}

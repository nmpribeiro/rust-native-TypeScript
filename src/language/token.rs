use plain_enum::plain_enum_mod;
use std::fmt;

plain_enum_mod! {this,TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    // 10
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    // 20
    String,
    Number,
    //Operations
    And,
    Or,
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators
    BitwiseAnd, // a & b
    BitwiseOr,  // a | b
    BitwiseXor, // a ^ b
    BitwiseNot, // ~ a
    // Keywords.
    Class,
    Else,
    // 30
    Extends,
    False,
    For,
    Function,
    If,
    Null,
    Print,
    Return,
    Super,
    This,
    // 40
    True,
    Var,
    While,

    Error,
    EOF,
}}

#[derive(Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: isize,
    pub lexeme: String,
    pub error: Option<String>,
}

impl Token {
    pub fn generic_token(token: TokenType) -> Token {
        Token {
            t_type: token,
            start: 0,
            length: 0,
            line: 0,
            lexeme: String::from(""),
            error: None,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "TokenType::LEFT_PAREN"),
            TokenType::RightParen => write!(f, "TokenType::RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "TokenType::LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "TokenType::RIGHT_BRACE"),
            TokenType::Comma => write!(f, "TokenType::COMMA"),
            TokenType::Dot => write!(f, "TokenType::DOT"),
            TokenType::Minus => write!(f, "TokenType::MINUS"),
            TokenType::Plus => write!(f, "TokenType::PLUS"),
            TokenType::Semicolon => write!(f, "TokenType::SEMICOLON"),
            TokenType::Slash => write!(f, "TokenType::SLASH"),
            TokenType::Star => write!(f, "TokenType::STAR"),
            TokenType::Bang => write!(f, "TokenType::BANG"),
            TokenType::BangEqual => write!(f, "TokenType::BANG_EQUAL"),
            TokenType::Equal => write!(f, "TokenType::EQUAL"),
            TokenType::EqualEqual => write!(f, "TokenType::EQUAL_EQUAL"),
            TokenType::Greater => write!(f, "TokenType::GREATER"),
            TokenType::GreaterEqual => write!(f, "TokenType::GREATER_EQUAL"),
            TokenType::Less => write!(f, "TokenType::LESS"),
            TokenType::LessEqual => write!(f, "TokenType::LESS_EQUAL"),
            TokenType::Identifier => write!(f, "TokenType::IDENTIFIER"),
            TokenType::String => write!(f, "TokenType::STRING"),
            TokenType::Number => write!(f, "TokenType::NUMBER"),
            TokenType::And => write!(f, "TokenType::AND"),
            TokenType::Class => write!(f, "TokenType::CLASS"),
            TokenType::Else => write!(f, "TokenType::ELSE"),
            TokenType::Extends => write!(f, "TokenType::EXTENDS"),
            TokenType::False => write!(f, "TokenType::FALSE"),
            TokenType::For => write!(f, "TokenType::FOR"),
            TokenType::Function => write!(f, "TokenType::FUN"),
            TokenType::If => write!(f, "TokenType::IF"),
            TokenType::Null => write!(f, "TokenType::NULL"),
            TokenType::Or => write!(f, "TokenType::OR"),
            TokenType::Print => write!(f, "TokenType::PRINT"),
            TokenType::Return => write!(f, "TokenType::RETURN"),
            TokenType::Super => write!(f, "TokenType::SUPER"),
            TokenType::This => write!(f, "TokenType::THIS"),
            TokenType::True => write!(f, "TokenType::TRUE"),
            TokenType::Var => write!(f, "TokenType::VAR"),
            TokenType::While => write!(f, "TokenType::WHILE"),
            TokenType::Error => write!(f, "TokenType::ERROR"),
            TokenType::EOF => write!(f, "TokenType::EOF"),
            TokenType::BitwiseAnd => write!(f, "TokenType::BITWISE_AND"), // a & b
            TokenType::BitwiseOr => write!(f, "TokenType::BITWISE_OR"),   // a | b
            TokenType::BitwiseXor => write!(f, "TokenType::BITWISE_XOR"), // a ^ b
            TokenType::BitwiseNot => write!(f, "TokenType::BITWISE_NOT"), // ~ a
        }
    }
}

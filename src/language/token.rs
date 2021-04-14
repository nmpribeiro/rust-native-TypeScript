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
    String,
    Number,

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
    // pub source: String,
    pub error: Option<String>, // &'static str,
}

impl Token {
    pub fn generic_token(token: TokenType) -> Token {
        Token {
            t_type: token,
            start: 0,
            length: 0,
            line: 0,
            lexeme: String::from(""),
            // source: String::from(""),
            error: None,
        }
    }

    // pub fn generic_ident(text: &str) -> Token {
    //     Token {
    //         t_type: Type::Identifier,
    //         lexeme: interner::intern(text),
    //         line: 0,
    //     }
    // }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "TOKEN_TYPE_LEFT_PAREN"),
            TokenType::RightParen => write!(f, "TOKEN_TYPE_RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "TOKEN_TYPE_LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "TOKEN_TYPE_RIGHT_BRACE"),
            TokenType::Comma => write!(f, "TOKEN_TYPE_COMMA"),
            TokenType::Dot => write!(f, "TOKEN_TYPE_DOT"),
            TokenType::Minus => write!(f, "TOKEN_TYPE_MINUS"),
            TokenType::Plus => write!(f, "TOKEN_TYPE_PLUS"),
            TokenType::Semicolon => write!(f, "TOKEN_TYPE_SEMI_COLON"),
            TokenType::Slash => write!(f, "TOKEN_TYPE_SLASH"),
            TokenType::Star => write!(f, "TOKEN_TYPE_STAR"),
            TokenType::Bang => write!(f, "TOKEN_TYPE_BANG"),
            TokenType::BangEqual => write!(f, "TOKEN_TYPE_BANG_EQUAL"),
            TokenType::Equal => write!(f, "TOKEN_TYPE_EQUAL"),
            TokenType::EqualEqual => write!(f, "TOKEN_TYPE_EQUAL_EQUAL"),
            TokenType::Greater => write!(f, "TOKEN_TYPE_GREATER"),
            TokenType::GreaterEqual => write!(f, "TOKEN_TYPE_GREATER_EQUAL"),
            TokenType::Less => write!(f, "TOKEN_TYPE_LESS"),
            TokenType::LessEqual => write!(f, "TOKEN_TYPE_LESS_EQUAL"),
            TokenType::Identifier => write!(f, "TOKEN_TYPE_IDENTIFIER"),
            TokenType::String => write!(f, "TOKEN_TYPE_STRING"),
            TokenType::Number => write!(f, "TOKEN_TYPE_NUMBER"),
            TokenType::And => write!(f, "TOKEN_TYPE_AND"),
            TokenType::Class => write!(f, "TOKEN_TYPE_CLASS"),
            TokenType::Else => write!(f, "TOKEN_TYPE_ELSE"),
            TokenType::Extends => write!(f, "TOKEN_TYPE_EXTENDS"),
            TokenType::False => write!(f, "TOKEN_TYPE_FALSE"),
            TokenType::For => write!(f, "TOKEN_TYPE_FOR"),
            TokenType::Function => write!(f, "TOKEN_TYPE_FUN"),
            TokenType::If => write!(f, "TOKEN_TYPE_IF"),
            TokenType::Null => write!(f, "TOKEN_TYPE_NULL"),
            TokenType::Or => write!(f, "TOKEN_TYPE_OR"),
            TokenType::Print => write!(f, "TOKEN_TYPE_PRINT"),
            TokenType::Return => write!(f, "TOKEN_TYPE_RETURN"),
            TokenType::Super => write!(f, "TOKEN_TYPE_SUPER"),
            TokenType::This => write!(f, "TOKEN_TYPE_THIS"),
            TokenType::True => write!(f, "TOKEN_TYPE_TRUE"),
            TokenType::Var => write!(f, "TOKEN_TYPE_VAR"),
            TokenType::While => write!(f, "TOKEN_TYPE_WHILE"),
            TokenType::Error => write!(f, "TOKEN_TYPE_ERROR"),
            TokenType::EOF => write!(f, "TOKEN_TYPE_EOF"),
            TokenType::BitwiseAnd => write!(f, "TOKEN_TYPE_BITWISE_AND"), // a & b
            TokenType::BitwiseOr => write!(f, "TOKEN_TYPE_BITWISE_OR"),   // a | b
            TokenType::BitwiseXor => write!(f, "TOKEN_TYPE_BITWISE_XOR"), // a ^ b
            TokenType::BitwiseNot => write!(f, "TOKEN_TYPE_BITWISE_NOT"), // ~ a
        }
    }
}

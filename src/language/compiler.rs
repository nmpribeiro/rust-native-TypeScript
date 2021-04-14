use plain_enum::{plain_enum_mod, TPlainEnum};

use super::chunk::Chunk;
#[cfg(feature = "log_level_debug")]
use crate::language::debug::Debug;
// use super::common::MutRc;
use super::parser::Parser;
#[cfg(feature = "log_level_debug")]
use super::scanner::Scanner;
use super::token::TokenType;
use {
    super::common::{ConstantIndex, OpCode},
    super::value::NumberValueType,
};

plain_enum_mod! {this,Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}}

struct ParseRule {
    prefix: Option<fn(&mut Compiler, bool)>,
    infix: Option<fn(&mut Compiler, bool)>,
    precedence: Precedence,
}

impl ParseRule {
    const fn new(precedence: Precedence) -> ParseRule {
        ParseRule {
            prefix: None,
            infix: None,
            precedence,
        }
    }

    const fn new_both(
        prefix: fn(&mut Compiler, bool),
        infix: Option<fn(&mut Compiler, bool)>,
        precedence: Precedence,
    ) -> ParseRule {
        ParseRule {
            prefix: Some(prefix),
            infix,
            precedence,
        }
    }

    const fn new_infix(infix: fn(&mut Compiler, bool), precedence: Precedence) -> ParseRule {
        ParseRule {
            prefix: None,
            infix: Some(infix),
            precedence,
        }
    }
}

// @implNote: it has to match the same number & position as TokenType
static RULES: [ParseRule; 45] = [
    ParseRule::new_both(
        |compiler, _| compiler.grouping(),
        Some(|compiler, _| compiler.call()),
        Precedence::Call,
    ), // LEFT_PAREN
    ParseRule::new(Precedence::None), // RIGHT_PAREN
    ParseRule::new(Precedence::None), // LEFT_BRACE
    ParseRule::new(Precedence::None), // RIGHT_BRACE
    ParseRule::new(Precedence::None), // COMMA
    ParseRule::new_infix(
        |compiler, can_assign| compiler.dot(can_assign),
        Precedence::Call,
    ), // DOT
    ParseRule::new_both(
        |compiler, _| compiler.unary(),
        Some(|compiler, _| compiler.binary()),
        Precedence::Term,
    ), // MINUS
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Term), // PLUS
    ParseRule::new(Precedence::None), // SEMICOLON
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Factor), // SLASH
    // 10
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Factor), // STAR
    ParseRule::new_both(|compiler, _| compiler.unary(), None, Precedence::None), // BANG
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Equality), // BANG_EQUAL
    ParseRule::new(Precedence::None),                                          // EQUAL
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Equality), // EQUAL_EQUAL
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Comparison), // GREATER
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Comparison), // GREATER_EQUAL
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Comparison), // LESS
    ParseRule::new_infix(|compiler, _| compiler.binary(), Precedence::Comparison), // LESS_EQUAL
    ParseRule::new_both(
        |compiler, can_assign| compiler.variable(can_assign),
        None,
        Precedence::None,
    ), // IDENTIFIER
    // 20
    ParseRule::new_both(|compiler, _| compiler.string(), None, Precedence::Term), // STRING
    ParseRule::new_both(|compiler, _| compiler.literal(), None, Precedence::None), // NUMBER
    ParseRule::new_infix(|compiler, _| compiler.and(), Precedence::And),          // AND
    ParseRule::new_infix(|compiler, _| compiler.or(), Precedence::Or),            // OR
    ParseRule::new(Precedence::None),                                             // BITWISE AND
    ParseRule::new(Precedence::None),                                             // BITWISE OR
    ParseRule::new(Precedence::None),                                             // BITWISE XOR
    ParseRule::new(Precedence::None),                                             // BITWISE NOT
    ParseRule::new(Precedence::None),                                             // CLASS
    ParseRule::new(Precedence::None),                                             // ELSE
    // 30
    ParseRule::new(Precedence::None), // EXTENDS
    ParseRule::new_both(|compiler, _| compiler.literal(), None, Precedence::None), // FALSE
    ParseRule::new(Precedence::None), // FOR
    ParseRule::new(Precedence::None), // FUNCTION
    ParseRule::new(Precedence::None), // IF
    ParseRule::new_both(|compiler, _| compiler.literal(), None, Precedence::None), // NULL
    ParseRule::new(Precedence::None), // PRINT
    ParseRule::new(Precedence::None), // RETURN
    ParseRule::new_both(|compiler, _| compiler.super_(), None, Precedence::None), // SUPER
    ParseRule::new_both(|compiler, _| compiler.this(), None, Precedence::None), // THIS
    // 40
    ParseRule::new_both(|compiler, _| compiler.literal(), None, Precedence::None), // TRUE
    ParseRule::new(Precedence::None),                                              // VAR
    ParseRule::new(Precedence::None),                                              // WHILE
    ParseRule::new(Precedence::None),                                              // ERROR
    ParseRule::new(Precedence::None),                                              // EOF
];

pub struct Compiler {
    parser: Parser,
    compiling_chunk: Chunk,
    // function: Function,
    // function_type: FunctionType,

    // locals: Vec<Local>,
    // scope_depth: usize,
    // upvalues: SmallVec<[Upvalue; 3]>,

    // enclosing: Option<Box<Compiler>>,
    // class_stack: MutRc<Vec<ClassCompile>>,
}

impl Compiler {
    pub fn new(code: &str) -> Compiler {
        Compiler {
            parser: Parser::new(code),
            compiling_chunk: Chunk::new(),
        }
    }

    // pub fn compile(&mut self, source: String, chunk: &Chunk) {
    pub fn compile(&mut self, chunk: Chunk) {
        self.parser.advance();
        self.compiling_chunk = chunk;

        while !self.parser.match_next(TokenType::EOF) {
            self.declaration();
        }
        let function = self.end_compiliation();
        // if self.parser_mut().had_error {
        //     None
        // } else {
        //     Some(Rc::new(Closure {
        //         function,
        //         upvalues: SmallVec::new(),
        //     }))
        // }
    }

    fn expression(&mut self) {
        println!("expression");
        self.parse_precedence(Precedence::Assignment);
    }

    fn declaration(&mut self) {
        match () {
            // _ if self.parser.match_next(TokenType::Class) => self.class_declaration(),
            // _ if self.parser.match_next(TokenType::Var) => self.var_declaration(),
            // _ if self.parser.match_next(TokenType::Fun) => self.fun_declaration(),
            _ => self.statement(),
        }

        self.parser.synchronize();
    }

    fn end_compiliation(&mut self) {
        self.emit_return();
        // Rc::new(RefCell::new(mem::replace(
        //     &mut self.function,
        //     Self::new_function(None, 0),
        // )))
    }

    fn statement(&mut self) {
        match () {
            _ if self.parser.match_next(TokenType::Print) => self.print_statement(),
            // _ if self.parser.match_next(TokenType::If) => self.if_statement(),
            // _ if self.parser.match_next(TokenType::While) => self.while_statement(),
            // _ if self.parser.match_next(TokenType::For) => self.for_statement(),
            // _ if self.parser.match_next(TokenType::Return) => self.return_statement(),
            // _ if self.parser.match_next(TokenType::LeftBrace) => {
            //     self.begin_scope();
            //     self.block();
            //     self.end_scope();
            // }
            _ => self.expression_statement(),
        };
    }

    fn print_statement(&mut self) {
        println!("print_statement");
        self.expression();
        self.consume(TokenType::Semicolon, "Expected ';' after value.");
        self.emit_byte(OpCode::PRINT);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expected ';' after expression.");
        self.emit_byte(OpCode::POP);
    }

    fn error(&mut self, message: &str) {
        self.parser.error(message.to_string());
    }

    pub fn consume(&mut self, t_type: TokenType, message: &str) {
        if self.parser.get_current().t_type == t_type {
            self.parser.advance();
            return;
        }
        self.parser.error(message.to_string());
    }

    // Emition
    pub fn emit_byte(&mut self, op_code: OpCode) {
        self.compiling_chunk
            .add_op_code(op_code, self.parser.scanner.line);
    }

    pub fn emit_bytes(&mut self, op_code1: OpCode, op_code2: OpCode) {
        self.emit_byte(op_code1);
        self.emit_byte(op_code2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::RETURN);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        #[cfg(feature = "log_level_debug")]
        if !self.parser.had_error {
            self.compiling_chunk.disassemble("code");
        }
    }

    fn binary(&mut self) {
        println!("binary");
        // Remember the operator.
        let operator_type: TokenType = self.parser.get_previous().t_type;
        // Compile the right operand.
        let rule = Compiler::get_rule(operator_type);
        unsafe {
            self.parse_precedence(Precedence::from_usize(rule.precedence.to_usize() + 1));
        }
        // Emit the operator instruction.
        match operator_type {
            TokenType::BangEqual => self.emit_bytes(OpCode::EQUAL, OpCode::NOT),
            TokenType::EqualEqual => self.emit_byte(OpCode::EQUAL),
            TokenType::Greater => self.emit_byte(OpCode::GREATER),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::LESS, OpCode::NOT),
            TokenType::Less => self.emit_byte(OpCode::LESS),
            TokenType::LessEqual => self.emit_bytes(OpCode::GREATER, OpCode::NOT),
            TokenType::Plus => self.emit_byte(OpCode::ADD),
            TokenType::Minus => self.emit_byte(OpCode::SUBTRACT),
            TokenType::Star => self.emit_byte(OpCode::MULTIPLY),
            TokenType::Slash => self.emit_byte(OpCode::DIVIDE),
            _ => return,
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value: f64 = self
            .parser
            .get_previous()
            .lexeme
            .parse::<f64>()
            .expect("Not a number!");
        self.compiling_chunk.add_constant(value);
    }

    fn unary(&mut self) {
        println!("unary");
        let operator_type: TokenType = self.parser.get_previous().t_type;
        // Compile the operand.
        self.parse_precedence(Precedence::Unary);
        // Emit the operator instruction.
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::NEGATE),
            TokenType::Bang => self.emit_byte(OpCode::NOT),
            _ => return, // Unreachable
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        println!("parse_precedence");
        self.parser.advance();
        let prefix_rule = Compiler::get_rule(self.parser.get_previous().t_type).prefix;
        let can_assign = precedence <= Precedence::Assignment;
        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self, can_assign); // is prefixRule();
        } else {
            self.error("Expected expression.");
            return;
        }

        while precedence.to_usize()
            <= Compiler::get_rule(self.parser.get_current().t_type)
                .precedence
                .to_usize()
        {
            // do stuf
            self.parser.advance();
            let infix_rule = Compiler::get_rule(self.parser.get_previous().t_type).infix;
            match infix_rule {
                Some(infix_rule_exec) => infix_rule_exec(self, can_assign),
                None => self.error("Unexpected infix expression"),
            }
        }

        // if (can_assign && self.match_next(TokenType::Equal)) {
        //     self.error("Invalid assignment target.");
        //     self.expression();
        // }
    }

    fn get_rule(t_type: TokenType) -> &'static ParseRule {
        println!("get_rule t_type::{} {}", t_type, t_type.to_usize());
        &RULES[t_type.to_usize() - 1]
    }

    fn dot(&mut self, _can_assign: bool) {}
    fn variable(&mut self, _can_assign: bool) {}
    fn string(&mut self) {}
    fn literal(&mut self) {
        match self.parser.get_previous().t_type {
            TokenType::False => self.emit_byte(OpCode::FALSE),
            TokenType::Null => self.emit_byte(OpCode::NULL),
            TokenType::True => self.emit_byte(OpCode::TRUE),
            _ => return,
        }
    }

    fn and(&mut self) {}
    fn or(&mut self) {}
    fn super_(&mut self) {}
    fn this(&mut self) {}
    fn call(&mut self) {}

    // In order to debug the scanner, just hook this up from outside :)
    #[cfg(feature = "log_level_debug")]
    pub fn debugScanner(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        let mut line: isize = -1; //it could be any value > 0, really
        loop {
            let token = scanner.scan_token();
            if token.line != line {
                print!("{:04} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }

            let lexeme = match token.t_type {
                TokenType::Error => token.error.unwrap(),
                _ => token.lexeme,
            };
            println!("{} {}", token.t_type, lexeme);

            match token.t_type {
                TokenType::EOF => break,
                _ => {}
            }
        }
    }
}

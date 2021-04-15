use rustc_hash::FxHasher;
use smol_str::SmolStr;
use std::fmt;
use std::{cell::RefCell, hash::BuildHasherDefault, rc::Rc};
use string_interner::{backend::BucketBackend, symbol::SymbolU32, StringInterner};

pub type MutRc<T> = Rc<RefCell<T>>;
pub type StrId = SymbolU32;

thread_local! {
    static INTERN: RefCell<StringInterner::<StrId, BucketBackend<StrId>, BuildHasherDefault<FxHasher>>> = RefCell::new(StringInterner::with_capacity(500));
}

/// Return interned variant of given string
pub fn intern<T: AsRef<str>>(of: T) -> StrId {
    INTERN.with(|i| i.borrow_mut().get_or_intern(of))
}

/// Return string of interned id
pub fn to_str(of: StrId) -> SmolStr {
    INTERN.with(|i| SmolStr::new(i.borrow().resolve(of).unwrap()))
}

#[derive(Copy, Clone)]
pub enum OpCode {
    NULL,
    // Const opcodes - push the given value the stack
    BOOL(bool),
    NUMBER(f64),
    STRING(StrId),
    // Operations
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NEGATE,
    RETURN,
    //
    TRUE,
    FALSE,

    NOT,
    POP,

    EQUAL,
    GREATER,
    LESS,

    PRINT,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpCode::NULL => write!(f, "OP_NULL"),
            OpCode::BOOL(bool) => write!(f, "OP_BOOL:{}", bool),
            OpCode::NUMBER(f64) => write!(f, "OP_NUMBER:{}", f64),
            OpCode::STRING(StrId) => write!(f, "OP_STRING:{}", to_str(StrId)),
            OpCode::ADD => write!(f, "OP_ADD"),
            OpCode::SUBTRACT => write!(f, "OP_SUBTRACT"),
            OpCode::MULTIPLY => write!(f, "OP_MULTIPLY"),
            OpCode::DIVIDE => write!(f, "OP_DIVIDE"),
            OpCode::NEGATE => write!(f, "OP_NEGATE"),
            OpCode::RETURN => write!(f, "OP_RETURN"),
            OpCode::TRUE => write!(f, "OP_TRUE"),
            OpCode::FALSE => write!(f, "OP_FLASE"),
            OpCode::NOT => write!(f, "OP_NOT"),
            OpCode::POP => write!(f, "OP_POP"),
            OpCode::EQUAL => write!(f, "OP_EQUAL"),
            OpCode::GREATER => write!(f, "OP_GREATER"),
            OpCode::LESS => write!(f, "OP_LESS"),
            OpCode::PRINT => write!(f, "OP_PRINT"),
        }
    }
}

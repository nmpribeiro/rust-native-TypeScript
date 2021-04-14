use super::common::{to_str, StrId};
use enum_methods::EnumAsGetters;
use enum_methods::EnumIntoGetters;
use enum_methods::EnumIsA;
use std::{fmt, mem::discriminant, rc::Rc};

pub type NumberValueType = f64;

#[derive(Debug, Clone, PartialEq, EnumAsGetters, EnumIsA, EnumIntoGetters)]
pub enum Value {
    ValBool(bool),
    ValNull,
    ValNumber(f64),
    ConstString(StrId),
    DynString(Rc<str>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::ValBool(value) => !*value,
            Value::ValNull => true,
            _ => false,
        }
    }
    pub fn same_type_as(&self, other: &Value) -> bool {
        discriminant(self) == discriminant(other)
    }

    pub fn less(&self, other: Value) -> Option<Value> {
        if self.same_type_as(&other) && self.is_val_number() {
            Some(Value::ValBool(self.as_val_number() < other.as_val_number()))
        } else {
            None
        }
    }

    pub fn greater(&self, other: Value) -> Option<Value> {
        if self.same_type_as(&other) && self.is_val_number() {
            Some(Value::ValBool(self.as_val_number() > other.as_val_number()))
        } else {
            None
        }
    }

    pub fn add(self, other: Value) -> Value {
        match (&self, &other) {
            (Value::ValNumber(s), Value::ValNumber(o)) => Value::ValNumber(s + o),
            _ => Value::DynString(Rc::from((self.to_string() + &other.to_string()).as_str())),
        }
    }

    pub fn sub(self, other: Value) -> Option<Value> {
        match (&self, &other) {
            (Value::ValNumber(s), Value::ValNumber(o)) => Some(Value::ValNumber(s - o)),
            _ => None,
        }
    }

    pub fn mul(self, other: Value) -> Option<Value> {
        match (&self, &other) {
            (Value::ValNumber(s), Value::ValNumber(o)) => Some(Value::ValNumber(s * o)),

            (_, Value::ValNumber(o)) if *o > 0.0 => {
                let string = self.to_string();
                Some(Value::DynString(Rc::from(string.repeat(*o as usize))))
            }

            _ => None,
        }
    }

    pub fn div(self, other: Value) -> Option<Value> {
        match (&self, &other) {
            (Value::ValNumber(s), Value::ValNumber(o)) => Some(Value::ValNumber(s / o)),
            _ => None,
        }
    }

    pub fn neg(self) -> Option<Value> {
        match self {
            Value::ValNumber(value) => Some(Value::ValNumber(-value)),
            _ => None,
        }
    }

    pub fn not(self) -> Value {
        Value::ValBool(self.is_falsey())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::ValBool(val) => write!(f, "{}", val),
            Value::ValNull => write!(f, "null"),
            Value::ValNumber(val) => write!(f, "{}", val),
            Value::ConstString(val) => write!(f, "{}", to_str(*val)),
            Value::DynString(val) => write!(f, "{}", val),
        }
    }
}

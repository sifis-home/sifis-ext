use std::{
    borrow::Cow,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Condition {
    /// A simple way to express value equality.
    Value(Value),

    /// A custom conditional expression.
    Expr(Expr),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// A boolean value.
    Bool(bool),

    /// A numeric value.
    Number(serde_json::Number),

    /// A string value.
    String(Cow<'static, str>),
}

impl From<bool> for Value {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for Value {
    #[inline]
    fn from(value: String) -> Self {
        Self::String(Cow::Owned(value))
    }
}

impl From<&'static str> for Value {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self::String(Cow::Borrowed(value))
    }
}

impl From<serde_json::Number> for Value {
    #[inline]
    fn from(value: serde_json::Number) -> Self {
        Self::Number(value)
    }
}

macro_rules! impl_value_from {
    ($($ty:ty)+) => {
        $(
            impl From<$ty> for Value {
                #[inline]
                fn from(value: $ty) -> Self {
                    Self::Number(value.into())
                }
            }
        )+
    };
}

impl_value_from!(i8 u8 i16 u16 i32 u32 i64 u64 isize usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidFloat;

impl Display for InvalidFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("infinite or NaN are not valid JSON numbers")
    }
}

impl std::error::Error for InvalidFloat {}

impl TryFrom<f32> for Value {
    type Error = InvalidFloat;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(f64::from(value))
    }
}

impl TryFrom<f64> for Value {
    type Error = InvalidFloat;

    #[inline]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        serde_json::Number::from_f64(value)
            .ok_or(InvalidFloat)
            .map(Self::Number)
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Expr {
    /// The value to be compared.
    #[serde(rename = "sho:value")]
    pub value: Value,

    /// The operation to be used in order to evaluate the [`value`].
    ///
    /// [`value`]: Expr::value
    #[serde(rename = "sho:op")]
    pub op: Operation,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    /// Less than.
    Lt,

    /// Less or equal then.
    Le,

    /// Not equal to.
    Ne,

    /// Greater than.
    Gt,

    /// Greater or equal than.
    Ge,
}

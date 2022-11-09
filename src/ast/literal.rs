use serde::{Deserialize, Serialize};


/// Scalar float values can be written as literal integer or floating-point numbers in the format.
#[derive(Debug, Clone, Copy, PartialOrd, Deserialize, Serialize)]
pub struct NumberLiteral{
    pub value: f64
}

impl PartialEq for NumberLiteral{
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() <= f64::EPSILON
    }
}

impl Eq for NumberLiteral {}

impl NumberLiteral{
    pub fn new(value: f64) -> Self{
        Self { value }
    }
}


/// StringLiteral is a literal in single quotes, double quotes or backticks. No escaping is processed inside backticks.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct StringLiteral{
    pub value: String,
}


impl StringLiteral{
    pub fn new(value: String) -> Self{
        Self { value }
    }
}
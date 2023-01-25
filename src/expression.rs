use std::fmt::{Display, Formatter};
use std::str::Chars;

use crate::valid_types::{ValidTypesMapper};

#[derive(Debug)]
pub struct Expression {
    pub raw_expression: String,
}

static ALLOWED_CHARACTERS: &str = "0123456789+-*/^()";

impl Expression {
    pub fn new(raw_expression: String) -> Expression {
        Expression { raw_expression }
    }

    pub fn evaluate(&self) -> i32 {
        let processed_expression = Expression::process_chars(self.raw_expression.trim().chars());
        Expression::translate(&processed_expression)
    }

    fn process_chars(chars: Chars) -> String {
        return chars.filter(|c| ALLOWED_CHARACTERS.contains(*c)).collect();
    }

    fn translate(str: &String) -> i32 {
        let mut parser: ValidTypesMapper = ValidTypesMapper::new(str);
        parser.parse();
        println!("{:?}", parser);
        parser.evaluate()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "raw_expression: {}", self.raw_expression)
    }
}
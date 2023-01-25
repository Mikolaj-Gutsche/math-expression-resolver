use crate::expression::Expression;

mod expression;
mod valid_types;

fn main() {
    loop {
        let mut input: String = String::new();
        println!("Input math expression: ");
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        println!("Expression: {}", input);
        let exp: Expression = Expression::new(input);
        println!("{} = {}", exp.raw_expression, exp.evaluate());
    }
}

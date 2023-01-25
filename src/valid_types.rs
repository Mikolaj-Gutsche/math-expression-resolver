
#[derive(PartialEq)]
#[derive(Debug,Copy,Clone)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(PartialEq)]
#[derive(Debug,Copy,Clone)]
pub enum BracketType {
    Open,
    Closed,
}

#[derive(PartialEq)]
#[derive(Debug,Copy,Clone)]
pub enum ValidTypes {
    Character(char),
    Number(i32),
    Operator(OperatorType),
    Bracket(BracketType),
}

#[derive(Debug, Clone)]
pub struct ValidTypesMapper {
    data: Vec<ValidTypes>
}

impl OperatorType {
    pub fn get_importance(&self) -> i32 {
        match self {
            OperatorType::Add => 1,
            OperatorType::Sub => 1,
            OperatorType::Mul => 2,
            OperatorType::Div => 2,
            OperatorType::Pow => 3
        }
    }

    pub fn get_operation(&self) -> Box<dyn Fn(i32, i32) -> i32> {
        match self {
            OperatorType::Add => Box::new(|a, b| a+b),
            OperatorType::Sub => Box::new(|a, b| a-b),
            OperatorType::Mul => Box::new(|a, b| a*b),
            OperatorType::Div => Box::new(|a, b| a/b),
            OperatorType::Pow => Box::new(|a, b| a.pow(b as u32))
        }
    }
}

impl ValidTypesMapper {
    pub fn new(str: &String) -> ValidTypesMapper {
        let vec: Vec<ValidTypes> = str.chars().map(|c| ValidTypes::from(c)).collect();
        ValidTypesMapper { data: vec }
    }

    pub fn parse(&mut self) {
        let mut processed_data: Vec<ValidTypes> = Vec::new();
        let mut str: String = String::new();
        for valid_type in self.data.iter_mut() {
            if let ValidTypes::Character(c) = valid_type {
                if c.is_alphanumeric() {
                    str.push(*c);
                } else {
                    if !str.is_empty() {
                        let num = str.parse::<i32>().unwrap();
                        processed_data.push(ValidTypes::Number(num));
                        str.clear();
                    }
                    processed_data.push(
                        match *c {
                            '+' => ValidTypes::Operator(OperatorType::Add),
                            '-' => ValidTypes::Operator(OperatorType::Sub),
                            '*' => ValidTypes::Operator(OperatorType::Mul),
                            '/' => ValidTypes::Operator(OperatorType::Div),
                            '^' => ValidTypes::Operator(OperatorType::Pow),
                            '(' => ValidTypes::Bracket(BracketType::Open),
                            ')' => ValidTypes::Bracket(BracketType::Closed),
                            _ => ValidTypes::Character(*c)
                        });
                }
            } else {
                processed_data.push(*valid_type);
            }
        }
        if !str.is_empty() {
            let num = str.parse::<i32>().unwrap();
            processed_data.push(ValidTypes::Number(num));
        }

        self.data = processed_data;
    }

    pub fn evaluate(&mut self) -> i32 {
        while self.data.len() > 1 {
            let mut i : i32 = self.get_most_important_operation() as i32;

            let a = match self.data.get((i-1) as usize).unwrap() {
                ValidTypes::Number(num) => num,
                _ => &0
            };

            let op = match self.data.get((i) as usize).unwrap() {
                ValidTypes::Operator(a) => a,
                _ => &OperatorType::Add
            };

            let b = match self.data.get((i+1) as usize).unwrap() {
                ValidTypes::Number(num) => num,
                _ => &0
            };

            let result = ValidTypes::Number(op.get_operation()(*a, *b));
            if self.is_in_bracket(i) {
                for j in [2,-2] {
                    self.data.remove((i + j) as usize);
                }
                i -= 1;
            }

            for j in [1,0,-1] {
                self.data.remove((i + j) as usize);
            }

            self.data.insert((i - 1) as usize, result);
        }

        match self.data.first() {
            None => 0,
            Some(ValidTypes::Number(result)) => *result,
            _ => 0
        }
    }

    fn is_in_bracket(&self, pos: i32) -> bool {
        for i in [(-2, ValidTypes::Bracket(BracketType::Open)), (2, ValidTypes::Bracket(BracketType::Closed))] {
            return match self.data.get((pos + i.0) as usize) {
                None => false,
                Some(b) => *b == i.1
            }
        }

        false
    }

    fn get_most_important_operation(&self) -> usize {
        let mut depth = 0;
        let mut most_important = (0, 0);
        for (pos, e) in self.data.iter().enumerate() {
            let importance = match e {
                ValidTypes::Operator(o) => o.get_importance(),
                ValidTypes::Bracket(BracketType::Open) => {depth += 10; 0},
                ValidTypes::Bracket(BracketType::Closed) => {depth -= 10; 0}
                _ => 0
            };

            if (depth+importance) > most_important.1 {
                most_important = (pos, (depth+importance));
            }
        }

        most_important.0
    }
}


impl From<char> for ValidTypes {
    fn from(value: char) -> Self {
        ValidTypes::Character(value)
    }
}
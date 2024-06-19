use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut expression: String = String::new();
    if args.len() > 1 && args[1] == "-text" && args.len() > 2 {
        expression = args[2..].join(" ");
    } else {
        eprintln!("Usage: program -text <your_text>");
        return;
    }

    //Convert Expression to Vector of Tokens
    let tokens: Vec<Token> = tokenize(expression);
    println!("Tokens: {:?}", tokens);

    //Sort the Tokens in to Reverse Polish Notation
    let rpn: Vec<Token> = infix_to_rpn(tokens.clone());
    println!("RPN: {:?}", rpn);

    //Evaluate the Expression
    match evaluate_rpn(rpn) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}

#[derive(Debug, Clone)]
enum Token {
    Digit(String),
    Operator(String),
    LeftParen,
    RightParen,
}

fn tokenize(expression: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token: String = String::new();

    for c in expression.chars() {
        if c.is_whitespace() {
            continue;
        } else if c.is_digit(10) {
            current_token.push(c);
        } else {
            if !current_token.is_empty() {
                tokens.push(Token::Digit(current_token.clone()));
                current_token.clear();
            }

            match c {
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '+' | '-' | '*' | '/' => tokens.push(Token::Operator(c.to_string())),
                _ => eprintln!("Invalid character: {}", c),
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(Token::Digit(current_token));
    }

    tokens
}

fn infix_to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Digit(_) => output.push(token),
            Token::Operator(_) => {
                while let Some(op) = operators.last() {
                    if let Token::Operator(_) = op {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::LeftParen => operators.push(token),
            Token::RightParen => {
                while let Some(op) = operators.last() {
                    if let Token::LeftParen = op {
                        break;
                    } else {
                        output.push(operators.pop().unwrap());
                    }
                }
                operators.pop(); // Remove the left parenthesis
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

fn evaluate_rpn(tokens: Vec<Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Digit(d) => {
                if let Ok(num) = d.parse::<f64>() {
                    stack.push(num);
                } else {
                    return Err(format!("Invalid number: {}", d));
                }
            }
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err("Insufficient operands for operator".to_string());
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match op.as_str() {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "*" => stack.push(a * b),
                    "/" => {
                        if b == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        stack.push(a / b);
                    }
                    _ => return Err(format!("Invalid operator: {}", op)),
                }
            }
            _ => return Err("Invalid token".to_string()),
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }

    Ok(stack[0])
}
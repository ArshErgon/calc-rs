use colored::*;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::collections::HashMap;

pub fn start_shell() {
    println!("{}", "Calculator Shell - Type 'exit' to quit".bold().cyan());
    println!("{}", "Examples:".bold());
    println!("  {}", "PI = 3.141".yellow());
    println!("  {}", "x = 10".yellow());
    println!("  {}", "y = 20".yellow());
    println!("  {}", "x + y".green());
    println!("  {}", "result = x * 2 + y / 5".green());
    println!("  {}", "vars (list all variables)".magenta());
    println!();

    let mut variables: HashMap<String, f64> = HashMap::new();
    variables.insert("PI".to_string(), std::f64::consts::PI);

    let mut rl = Editor::<(), DefaultHistory>::new().expect("Failed to create editor");

    loop {
        let readline = rl.readline(&format!("{}", ">>> ".bright_blue()));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).ok();
                let command = line.trim();

                if command.eq_ignore_ascii_case("exit") {
                    println!("{}", "Exiting shell...".bold().cyan());
                    break;
                }
                if command.is_empty() {
                    continue;
                }

                if command.eq_ignore_ascii_case("vars") {
                    if variables.is_empty() {
                        println!("{}", "No variables set.".red());
                    } else {
                        for (key, val) in &variables {
                            println!("{} = {}", key.bold().yellow(), val.to_string().green());
                        }
                    }
                    continue;
                }

                // Assignment or expression
                if let Some(eq_pos) = command.find('=') {
                    let var_name = command[..eq_pos].trim();
                    let expression = command[eq_pos + 1..].trim();

                    if var_name.is_empty() {
                        println!("{}", "Error: Variable name cannot be empty".red());
                        continue;
                    }

                    if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        println!("{}", "Error: Invalid variable name".red());
                        continue;
                    }

                    match evaluate_expression(expression, &variables) {
                        Ok(value) => {
                            variables.insert(var_name.to_string(), value);
                            println!(
                                "{} = {}",
                                var_name.bold().yellow(),
                                value.to_string().green()
                            );
                        }
                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                    }
                } else {
                    match evaluate_expression(command, &variables) {
                        Ok(value) => println!("{}", value.to_string().green()),
                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", " (Ctrl+C) Use 'exit' to quit".yellow());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\n{}", "Exiting shell...".bold().cyan());
                break;
            }
            Err(err) => {
                eprintln!("{}", format!("Readline error: {:?}", err).red());
                break;
            }
        }
    }
}
fn evaluate_expression(expr: &str, variables: &HashMap<String, f64>) -> Result<f64, String> {
    let tokens = tokenize(expr)?;
    let tokens = substitute_vars(tokens, variables)?;
    evaluate_tokens(&tokens)
}

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(char),
    LParen,
    RParen,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match num_str.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => return Err(format!("Invalid number: {}", num_str)),
                }
            }
            '+' | '-' | '*' | '/' | '%' | '^' => {
                tokens.push(Token::Operator(ch));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // Variable name
                let mut var_name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                // Store as a special marker - we'll substitute later
                tokens.push(Token::Number(f64::NAN)); // placeholder
                tokens.push(Token::Operator('V')); // special marker
                                                   // Encode var name length
                tokens.push(Token::Number(var_name.len() as f64));
                for ch in var_name.chars() {
                    tokens.push(Token::Number(ch as u32 as f64));
                }
            }
            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }

    Ok(tokens)
}

fn substitute_vars(
    tokens: Vec<Token>,
    variables: &HashMap<String, f64>,
) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        if let Token::Operator('V') = tokens.get(i + 1).unwrap_or(&Token::LParen) {
            // This is a variable reference
            if let Token::Number(len) = tokens[i + 2] {
                let len = len as usize;
                let mut var_name = String::new();
                for j in 0..len {
                    if let Token::Number(ch_code) = tokens[i + 3 + j] {
                        var_name.push(char::from_u32(ch_code as u32).unwrap());
                    }
                }

                match variables.get(&var_name) {
                    Some(&value) => result.push(Token::Number(value)),
                    None => return Err(format!("Undefined variable: {}", var_name)),
                }

                i += 3 + len;
            } else {
                i += 1;
            }
        } else {
            result.push(tokens[i].clone());
            i += 1;
        }
    }

    Ok(result)
}

fn evaluate_tokens(tokens: &[Token]) -> Result<f64, String> {
    let mut output: Vec<f64> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    fn precedence(op: char) -> i32 {
        match op {
            '+' | '-' => 1,
            '*' | '/' | '%' => 2,
            '^' => 3,
            _ => 0,
        }
    }

    fn apply_op(a: f64, b: f64, op: char) -> Result<f64, String> {
        match op {
            '+' => Ok(a + b),
            '-' => Ok(a - b),
            '*' => Ok(a * b),
            '/' => {
                if b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
            '%' => Ok(a % b),
            '^' => Ok(a.powf(b)),
            _ => Err(format!("Unknown operator: {}", op)),
        }
    }

    fn process_operator(output: &mut Vec<f64>, op: char) -> Result<(), String> {
        if output.len() < 2 {
            return Err("Invalid expression".to_string());
        }
        let b = output.pop().unwrap();
        let a = output.pop().unwrap();
        output.push(apply_op(a, b, op)?);
        Ok(())
    }

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => output.push(*n),
            Token::Operator(op) => {
                while !operators.is_empty()
                    && precedence(*operators.last().unwrap()) >= precedence(*op)
                {
                    process_operator(&mut output, operators.pop().unwrap())?;
                }
                operators.push(*op);
            }
            Token::LParen => operators.push('('),
            Token::RParen => {
                while !operators.is_empty() && *operators.last().unwrap() != '(' {
                    process_operator(&mut output, operators.pop().unwrap())?;
                }
                if operators.is_empty() {
                    return Err("Mismatched parentheses".to_string());
                }
                operators.pop(); // Remove '('
            }
        }
        i += 1;
    }

    while !operators.is_empty() {
        let op = operators.pop().unwrap();
        if op == '(' {
            return Err("Mismatched parentheses".to_string());
        }
        process_operator(&mut output, op)?;
    }

    if output.len() != 1 {
        return Err("Invalid expression".to_string());
    }

    Ok(output[0])
}

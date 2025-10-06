use colored::*;
use std::collections::HashMap;


/// Shunting Yard Algorithm: Converts infix expression to RPN (postfix)

pub fn shunting_yard_algorithm(tokens: Vec<String>) -> (Vec<String>, String) {
    let mut output: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new(); // <-- changed from Vec<&str> to Vec<String>

    // Operator precedence
    let precedence: HashMap<&str, u8> = HashMap::from([("+", 1), ("-", 1), ("*", 2), ("/", 2)]);

    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                while let Some(op) = stack.last() {
                    if op != "(" && precedence[op.as_str()] >= precedence[token.as_str()] {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(token);
            }
            "(" => stack.push(token),
            ")" => {
                while let Some(op) = stack.pop() {
                    if op == "(" {
                        break;
                    }
                    output.push(op);
                }
            }
            _ => output.push(token), // numbers/operands
        }
    }

    // Pop remaining operators
    while let Some(op) = stack.pop() {
        output.push(op);
    }

    let rpn_string = output.join(" ");
    (output, rpn_string)
}

/// Reverse Polish Notation evaluator with colorful step-by-step output

/// Reverse Polish Notation evaluator with colorful step-by-step output
pub fn reverse_polish_notation(rpn_tokens: Vec<String>) {
    let mut stack: Vec<i128> = Vec::new();
    let mut step: usize = 1;

    // Step 1: Show RPN conversion
    println!("\n{}", "Step 1: Convert to RPN →".cyan().bold());
    println!("  {}\n", rpn_tokens.join(" ").yellow());

    for token in rpn_tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                if stack.len() < 2 {
                    println!("{}", "❌ Error: Not enough operands".red().bold());
                    return;
                }

                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0 {
                            println!("{}", "❌ Error: Division by zero".red().bold());
                            return;
                        } else {
                            a / b
                        }
                    }
                    _ => unreachable!(),
                };

                stack.push(result);

                println!(
                    "{} {} '{}' {} {} → {}",
                    format!("Step {}:", step).yellow().bold(),
                    "Apply".blue(),
                    token.blue().bold(),
                    "on".white(),
                    format!("{} and {}", a.to_string().green(), b.to_string().green()),
                    format!("Result: {}", result.to_string().magenta().bold())
                );

                println!(
                    "  Stack now: [{}]",
                    stack
                        .iter()
                        .map(|x| x.to_string().cyan().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );

                step += 1;
            }
            num_str => {
                // Parse the number
                let number: i128 = match num_str.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("{} '{}'", "❌ Invalid token:".red().bold(), num_str);
                        return;
                    }
                };

                stack.push(number);

                println!(
                    "{} {} {} → Stack now: [{}]",
                    format!("Step {}:", step).yellow().bold(),
                    "Push".blue(),
                    number.to_string().green().bold(),
                    stack
                        .iter()
                        .map(|x| x.to_string().cyan().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );

                step += 1;
            }
        }
    }

    // Final result
    if stack.len() == 1 {
        println!(
            "\n{} {}",
            "✅ Final Result:".cyan().bold(),
            stack[0].to_string().green().bold()
        );
    } else {
        println!("{}", "❌ Error: Invalid expression".red().bold());
    }
}

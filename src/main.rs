mod algorithms;
mod shell;
mod startup;
use colored::*;

fn main() {
    startup::starting_up_animated();
    startup::show_loading();
    // args is talking the cli argument in vector
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!(
            "{}\n\n{}:\n  {}\n\n{}:\n  {}\n  {}\n  {}\n",
            "⚠️  No command provided!".bold().red(),
            "Example".bold().green(),
            "cargo run shell".bright_blue(),
            "Commands".bold().cyan(),
            format!(
                "{} - {}",
                "shell".yellow(),
                "Interactive Shell for variable calculations".green()
            ),
            format!(
                "{} - {}",
                "algo OR algorithm".yellow(),
                "Convert an equation using Shunting Yard and\nsolve with Reverse Polish Notation"
                    .green()
            ),
            "\n Use one of the commands above to get started."
                .italic()
                .white()
        );

        std::process::exit(1); // clean exit instead of panic
    }

    let first_letter = args[0].clone();
    if !first_letter.is_empty() && first_letter.to_lowercase() == "shell" {
        shell::start_shell();
    } else if !first_letter.is_empty() && first_letter == "algo" || first_letter == "algorithm" {
        let input: String = args[1..].join(" ");
        let characters: Vec<String> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_string())
            .collect();
        // characters remove any white-spaces
        
        // shunting algorithm to convert numercial to polish notation
        let (number_vector, sya_number) = algorithms::shunting_yard_algorithm(characters);

        let msg = format!(" Numerical: {input} \n Shunting Algorithm: {sya_number}\n");
        println!("{msg}");

        algorithms::reverse_polish_notation(number_vector);
    } else {
        println!("Usage: program shell");
    }
}

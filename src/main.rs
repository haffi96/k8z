use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    verb: String,
    resource: Option<String>,
}

use inquire::Select;

fn run_action(resource: Option<String>) {
    println!("{:?}", resource);

    let options = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let ans = Select::new("Pick a fruit!", options).prompt();

    match ans {
        Ok(choice) => println!("{choice}! Nice"),
        Err(_) => println!("There was an error, try again!"),
    }
}

fn main() {
    let args = Cli::parse();

    match args.verb.as_str() {
        "get" => run_action(args.resource),
        _ => println!("Not found"),
    }
}

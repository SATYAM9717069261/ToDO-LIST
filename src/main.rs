mod todolist;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    key: String,
}

fn main() {
    let args = Cli::parse();
    let todo = todolist::TodoList::new();
    println!("Command line: {} {}", args.command, args.key);
}

mod todolist;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    key: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let mut todo = todolist::TodoList::new();
    let result = match args.command.as_str() {
        "add" => match args.key {
            Some(key) => {
                todo.add(key);
                Ok(())
            }
            None => Err("Key cannot be empty!".to_string()),
        },
        "mark-done" => match args.key {
            Some(key) => todo
                .mark(key, false)
                .map_err(|e| format!("Invalid key {}", e))
                .and(Ok(())),
            None => Err("Key cannot be empty!".to_string()),
        },
        "list" => {
            let (todo_items, done_items) = todo.list();

            println!("# TO DO");
            println!();
            todo_items.for_each(|x| println!(" * {}", x));

            println!();

            println!("# DONE");
            println!();
            done_items.for_each(|x| println!(" * {}", x));

            Ok(())
        }
        cmd => Err(format!("Command {} not recognised", cmd)),
    };

    match result {
        Err(e) => println!("ERROR: {}", e),
        Ok(_) => println!("SUCCESS"),
    }
}

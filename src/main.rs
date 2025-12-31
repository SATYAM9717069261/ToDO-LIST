mod todolist;
use std::io::{self, Write};

struct Cli {
    command: String,
    key: Option<String>,
}

fn main() {
    let mut todo = todolist::TodoList::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" || input == "exit" {
            println!("exit");
            break;
        }

        // split input like: add buy-milk
        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap();
        let key = parts.next();

        match command {
            "add" => {
                if let Some(k) = key {
                    todo.add(k.to_string());
                } else {
                    println!("Key missing");
                }
            }
            "done" => {
                if let Some(k) = key {
                    if let Err(e) = todo.mark(k.to_string(), false) {
                        println!("Error: {}", e);
                    }
                }
            }
            "list" => {
                let (todo_items, done_items) = todo.list();
                println!("TODO");
                todo_items.for_each(|x| println!(" -> {}", x));
                println!("DONE");
                done_items.for_each(|x| println!(" -> {}", x));
            }
            _ => println!("command Not Exist"),
        }
    }
}

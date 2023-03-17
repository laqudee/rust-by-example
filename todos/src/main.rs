use std::io;

fn main() {
    let mut todos = vec![];

    loop {
        println!("Enter a command (add, list, remove, quit):");

        let command = read_line();

        let command = command.trim();

        match command {
            "add" => {
                println!("Enter a todo:");

                let todo = read_line().trim().to_string();

                todos.push(todo);
            }
            "list" => {
                for (i, todo) in todos.iter().enumerate() {
                    println!("{}: {}", i, todo);
                }
            }
            "remove" => {
                if todos.len() == 0 {
                    println!("todos length is 0, cann't remove todo-item!");
                    continue;
                }
                println!("please enter index of todo-item you want: ");
                let remove_number = read_line().trim().to_string().parse().unwrap();
                todos.remove(remove_number);
                println!("Todos: {:?}", todos);
            }
            "quit" => break,
            _ => println!("Unknown command"),
        }
    }
}

fn read_line() -> String {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    value
}

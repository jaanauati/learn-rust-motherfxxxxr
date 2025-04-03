use serde::{Serialize, Deserialize};
mod input;
mod command_line;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum Status {
    InProgres,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    text: String,
    status: Option<Status>,
}

impl Todo {
    fn new(text: &str) -> Todo {
        Todo{
            text: String::from(text),
            status: None,
        }
    }
}

fn main() {
    use command_line::{get_command, Command};
    use input::get_text;
    let mut todos: Vec<Todo> = Vec::new();
    loop {
        let command = get_command();
        if let Command::List = command {
            println!("List:\n");
            for (idx, ref todo) in todos.iter().enumerate() {
                println!("{} -> {:?}", idx, todo);
            }
        } else if let Command::Done = command {
            println!("Mark Item as `Done`:\n");
            let todo_idx = get_text("Enter Item Number: ");
            let todo_idx : usize = todo_idx.parse().unwrap();
            if let Some(todo) = todos.get_mut(todo_idx) {
                let prev_status = todo.status;
                todo.status = Some(Status::Done);
                println!("{} -> (text={} previous={:?} -> {:?})", todo_idx, todo.text, prev_status, todo.status);
            }
        } else if let Command::InProgres = command {
            println!("Mark Item as `In Progress`:\n");
            let todo_idx = get_text("Enter Item Number: ");
            let todo_idx = todo_idx.parse::<usize>().unwrap();
            if let Some(todo) = todos.get_mut(todo_idx) {
                let prev_status = todo.status;
                todo.status = Some(Status::InProgres);
                println!("{} -> (text={} previous={:?} -> {:?})", todo_idx, todo.text, prev_status, todo.status);
            }
        } else if let Command::Add = command {
            println!("Add New Item:\n");
            let ref input = get_text("Enter item Text: ");
            if !input.is_empty() {
                todos.push(Todo::new(input));
            }
        } else if let Command::Save = command {
            let serialized_todos = serde_json::to_string(&todos);
            match serialized_todos {
                Ok(json) => { println!("{}", json);},
                _ => { println!("Error..."); }
            };
        }
    }
}

use inquire::{Select, InquireError};

#[derive(Debug)]
pub enum Command {
    List,
    Add,
    InProgres,
    Done,
    Save,
    Nil,
}

pub fn get_command() -> Command {
    let options: Vec<&str> = vec!["List", "Add", "Done", "Progress", "Save"];
    let ans: Result<&str, InquireError> = Select::new("\nSelect command: ", options).prompt();

    let cmd = | result: &str| match result.to_lowercase().as_str() {
        "list" => Command::List,
        "done" => Command::Done,
        "add" => Command::Add,
        "progress" => Command::InProgres,
        "save" => Command::Save,
        _ => Command::Nil,
    };

    let result = match ans {
        Ok(choice) => cmd(choice),
        Err(_) => Command::Nil,
    };
    return result;
}

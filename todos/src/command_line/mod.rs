use crate::input::get_text;

#[derive(Debug)]
pub enum Command {
    List,
    Add,
    InProgres,
    Done,
    Nil,
}

pub fn get_command() -> Command {
    let user_input = get_text("\nEnter option (list, add, progress, done): ");
    let result = match user_input.to_lowercase().as_str() {
        "list" => Command::List,
        "done" => Command::Done,
        "add" => Command::Add,
        "progress" => Command::InProgres,
        _ => Command::Nil,
    };
    print!("\x1Bc");
    return result;
}

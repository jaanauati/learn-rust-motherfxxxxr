use std::io;
use std::io::Write;
pub fn get_text(prompt: &str) -> String {
    let mut user_input = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut user_input);
    user_input.pop();
    return user_input;
}

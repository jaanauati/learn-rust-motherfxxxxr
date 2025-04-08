use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_text_with_reader<R: io::Read, W: io::Write>(prompt: &str, reader: &mut R, writer: &mut W) -> String {
    let mut buf_reader = BufReader::new(reader);
    let mut user_input = String::new();
    write!(writer, "{}", prompt).expect("Failed to write prompt");
    let _ = writer.flush();
    let _ = buf_reader.read_line(&mut user_input);
    user_input.pop();
    return user_input;
}

pub fn get_text(prompt: &str) -> String {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    get_text_with_reader(prompt, &mut stdin, &mut stdout)
}

#[cfg(test)]
mod tests;

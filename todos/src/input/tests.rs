
use super::*;
use std::io::Cursor;

#[test]
fn test_get_text_with_reader() {
    let mut mock_input = Cursor::new("test input\n");
    let mut mock_output: Vec<u8> = Vec::new();
    let result = get_text_with_reader("Enter text: ", &mut mock_input, &mut mock_output);
    assert_eq!(result, "test input");
    assert_eq!(String::from_utf8(mock_output).unwrap(), "Enter text: ");
}

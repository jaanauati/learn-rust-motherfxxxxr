use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_command_list() {
    assert_eq!(cmd("list"), Command::List);
}

#[test]
fn test_command_add() {
    assert_eq!(cmd("add"), Command::Add);
}

#[test]
fn test_command_done() {
    assert_eq!(cmd("done"), Command::Done);
}

#[test]
fn test_command_progress() {
    assert_eq!(cmd("progress"), Command::InProgres);
}

#[test]
fn test_command_save() {
    assert_eq!(cmd("save"), Command::Save);
}

#[test]
fn test_command_nil() {
    assert_eq!(cmd("unknown"), Command::Nil);
}

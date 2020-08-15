// extern crate we're testing, same as any other code would do.
extern crate adder;

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}

// importing common module.
mod common;

#[test]
fn test_add_with_setup() {
    // using common code.
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}

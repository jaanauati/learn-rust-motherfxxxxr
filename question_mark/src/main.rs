struct Bar {
    baz: String,   
}
struct Foo {
    bar: Option<Bar>,
}
struct Data {
    foo: Option<Foo>,
}

fn chaining_fail() -> Option<()> {
    let data = Data{foo: None};
    println!("baz {:?}", data.foo?.bar?.baz);
    // optional chaining in rust does not work as it does for javascript,
    // in the line above, the function execution is stopped and None will be
    // returned when any of the optional fields evualates to None.
    // So, the line below is never reached.
    None
}

fn chaining_ok() -> Option<()> {
    let data = Data{
        foo: Some(Foo{
            bar: Some(Bar{
                baz: String::from("baaaazzz!")
            })
        })
    };
    println!("baz {:?}", data.foo?.bar?.baz);
    None
}

use std::num::ParseIntError;
fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // returns an Err() immediately
    Ok(x + y)                    // Doesn't run.
}

fn parse_and_sum () -> Result<i32, ParseIntError> {
    let x = "40".parse::<i32>()?;
    let y = "2".parse::<i32>()?;
    return Ok(x + y);
}

fn main() {
    chaining_fail();
    chaining_ok();
    println!("try_to_parse: {:?}", try_to_parse());
    println!("parse_and_sum: {:?}", parse_and_sum().unwrap());
}
// output:
// baz "baaaazzz!"
// try_to_parse: Err(ParseIntError { kind: InvalidDigit })
// parse_and_sum: 42

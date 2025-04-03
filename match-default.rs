enum Foo {
    Bar,
    Baz,
}

fn main () {
    let g = Foo::Bar;
    let mut value = 32i32;

    value = match g {
        Foo::Baz => 99,
        _ => {},
    };
    println!("Hey yooo {}", value);
}

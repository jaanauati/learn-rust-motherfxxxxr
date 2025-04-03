fn foo(v: &mut i32) {
    *v += 1;
}

fn bar(v: &mut i32) {
    *v += 1;
}

fn main() {
    let mut var = 33;
    foo(&mut var);
    let mut inc = || {
        var +=1;
    };
    inc();
    bar(&mut var);
    println!("Hello, world! {}", var);
}

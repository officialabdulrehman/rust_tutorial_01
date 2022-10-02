use std::io;

fn main() {
    println!("Hello World!");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 1);
    println!("{}", input);
    hello();
}

fn hello() {
    println!("Hello there")
}

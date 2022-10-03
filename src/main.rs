// use std::io;

fn main() {
    // println!("Hello World!");
    // let mut input: String = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");

    // let int_input: i64 = input.trim().parse().unwrap();
    // println!("{}", int_input + 1);
    // println!("{}", input);
    // hello();

    // let cond: bool = 2 == 2;
    // println!("{}", cond);
    // if cond {
    //     println!("{}", cond);
    // } else {
    //     println!("{}", cond);
    // }

    // let cond2: bool = cond && false;
    // if cond2 {
    //     println!("Cond2 is true");
    // } else {
    //     println!("Cond2 is false");
    // }

    // if (cond2 && cond) || true {
    //     println!("composite if is true");
    // } else {
    //     println!("composite if is false");
    // }
    hello();
    print_the_sum(5, 5);

    let weird = {
        let x = 5;
        x + 5
    };
    println!("this is so weird {}", weird);

    let less_weird = return_sum(5, 5);
    println!("this is less weird {}", less_weird);

    let normal = return_sum_v2(5, 5);
    println!("this is normal {}", normal);

    let normal_v2 = return_sum_v3(5, 5);
    println!("this is normal_v2 {}", normal_v2);
}

fn hello() {
    println!("Hello there")
}

fn print_the_sum(x: i32, y: i32) {
    println!("The sum is = {}", x + y)
}

fn return_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn return_sum_v2(x: i32, y: i32) -> i32 {
    return x + y;
}

fn return_sum_v3(x: i32, y: i32) -> i32 {
    let result: i32 = x + y;
    return result;
}

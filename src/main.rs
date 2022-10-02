fn main() {
    let x: i32 = 4;
    println!("x is: {}", x);
    let mut x: i32 = 5;
    println!("x is: {}", x);
    x = 9;
    println!("x is: {}", x);
    let x: i32 = 20;
    println!("x is: {}", x);

    // learning

    let floating_point: f64 = 10.00;
    let letter: char = ';';
    let boolean: bool = true;

    let tup: (i32, bool, char) = (1, true, 'a');

    println!("{}", tup.1);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {}", array[0], array[1])
}

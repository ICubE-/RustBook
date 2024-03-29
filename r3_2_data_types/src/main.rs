use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Your guess is: {guess}");
    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;
    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");

    let t = true;
    let f: bool = false;
    println!("{t} {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("({x}, {y}, {z})");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("({five_hundred}, {six_point_four}, {one})");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let a = [3; 5];
    println!("{:?}", a);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first} {second}");

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}

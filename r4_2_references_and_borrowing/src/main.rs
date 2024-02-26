fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");
    change(&s);

    let mut s = String::from("hello");
    change_mut(&mut s);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // error: cannot borrow value as mutable multiple times
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1: {}", r1);
    }
    let r2 = &mut s;
    println!("r2: {}", r2);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // error: cannot borrow value as mutable if it is also borrowed as immutable
    // let r3 = &mut s;
    // println!("{}, {}, and {}", r1, r2, r3);

    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);

    // let reference_to_nothing = dangle();
    let _string = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &String) {
    // error: cannot modify borrowed immutable value
    // some_string.push_str(", world");
    println!("{}", some_string);
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}

// error: dangling reference
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
fn main() {
    let mut _s = String::new();
    
    let data = "initial contents";
    let _s = data.to_string();
    let _s = "initial contents".to_string();
    let _s = String::from("intial contents");
    
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    s.push('!');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Now s1 has been moved
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // Does not take ownership
    println!("{s}");
    let s = s1 + "-" + &s2 + "-" + &s3; // Takes ownership
    println!("{s}");

    let hello = String::from("Hola");
    let ho = &hello[0..2];
    let h = &hello[0..1];
    println!("{ho} {h}");
    let hello = "Здравствуйте";
    let ho = &hello[0..4];
    println!("{ho}");
    // error: not a char boundary
    // let _h = &hello[0..1];

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд". bytes() {
        println!("{b}");
    }
}

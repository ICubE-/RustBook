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

    
}

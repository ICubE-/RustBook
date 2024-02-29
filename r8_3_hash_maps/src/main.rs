fn main() {
    // Using hash map
    use std::collections::HashMap;

    // Creation
    let mut scores = HashMap::new();

    // Insertion
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // Iteration
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership change of key and value
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // Takes ownership
    // error: borrow of moved value
    // println!("{field_name}");

    // Update
    scores.insert(String::from("Blue"), 25); // Value updated: 10 -> 25
    println!("{:?}", scores);

    // Insertion only if value does not exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

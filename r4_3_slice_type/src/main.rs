fn main() {
    let mut s = String::from("hello world");
    let _word = first_word(&s);
    s.clear();
    // Now the `_word` is totally invalid!

    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    let mut s = String::from("hello world");
    let _word = first_word_slice(&s);
    s.clear();
    // error: cannot borrow value as mutable if it is also borrowed as immutable
    // println!("the first word is: {}", _word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

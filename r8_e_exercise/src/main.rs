use std::collections::HashMap;

fn main() {
    // 1. 
    // Given a list of integers, use a vector and return the median and mode of the list.
    let input = [0, 1, 1, 2, 3, 4, 4, 5, 6, 7, 7, 8, 8, 8, 9, 10, 10];

    let mut vec = Vec::from(input);
    vec.sort();
    let size = vec.len();
    let median = vec.get(size / 2).unwrap();

    let mut map = HashMap::new();
    for elt in input {
        let count = map.entry(elt).or_insert(0);
        *count += 1;
    }
    let mut vec: Vec<(&i32, &i32)> = {
        map
        .iter()
        .map(|(e, c)| (c, e))
        .collect()
    };
    vec.sort();
    vec.reverse();
    let mode = vec.first().unwrap().1;

    println!("median: {}, mode: {}", median, mode);


    // 2.
    // Convert strings to pig latin.
    // The first consonant of each word is moved to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!
    let input = ["first", "apple", "banana", "pig", "goat", "animal", "elephant"];
    let input = {
        input.iter()
        .map(|&s| String::from(s))
        .collect::<Vec<_>>()
    };

    fn make_pig_latin(s: &str) -> String {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];

        let mut is_starting_with_vowel = false;
        for v in vowels {
            if s.starts_with(v) {
                is_starting_with_vowel = true;
                break;
            }
        }

        if is_starting_with_vowel {
            format!("{}-hay", &s)
        } else {
            format!("{}-{}ay", &s[1..], &s[..1])
        }
    }

    let output = {
        input.iter()
        .map(|s| make_pig_latin(s))
        .collect::<Vec<_>>()
    };
    print!("{:?}", output);


    // 3.
    // Using a hash map and vectors, create a text interface to allow a user
    // to add employee names to a department in a company.
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or
    // all people in the company by department, sorted alphabetically.

}

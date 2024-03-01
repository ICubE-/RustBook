use core::panic;
use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    // Matching Result
    fn _open_greeting_file() {
        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }
    // _open_greeting_file();


    // Handling an error
    fn _open_greeting_file_create_if_not_found() {
        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        };
    }
    // _open_greeting_file_create_if_not_found();


    // unwrap shortcuts panic
    // let _greeting_file = File::open("hello.txt").unwrap();


    // expect also shortcuts panic, and sets panic message
    // let _greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");


    // Propagating an error
    fn _read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    // let _username = _read_username_from_file().unwrap();


    fn _read_username_from_file_shortcut()-> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    // let _username = _read_username_from_file_shortcut().unwrap();

    fn _read_username_from_file_shortcut_shorter()-> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    // let _username = _read_username_from_file_shortcut_shorter().unwrap();
}

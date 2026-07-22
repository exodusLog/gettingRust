use std::error;
use std::fs::File;
use std::io::{Error, ErrorKind};

pub fn result() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
pub fn another_res() {
    let file = File::open("hello.txt").unwrap_or_else(|error: Error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error: Error| panic!("problem creating file {:?}", error))
        } else {
            panic!("Problem opening file: {:?}", error)
        }
    });
}

extern crate rpassword;


use std::io::{self, BufRead};


// User inserts an integer value
// from the standart input (keyboard)
// and return this value.
pub fn read_integer() -> u32 {

    let mut key = 0;
    let mut number_as_text = String::new();

    println!("Enter the Encryption key: ");

        io::stdin()
            .read_line(&mut number_as_text)
            .expect("failed to read from stdin");

        let trimmed = number_as_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => key = i,
            Err(..) => println!("this was not an integer: {}", trimmed)
        };

    key

}


// User insterts a message (one line only is permitted)
// using the the standart input (keyboard)
// and return this message.
pub fn read_filename() -> String {

    let filename = rpassword::prompt_response_stdout("Enter filename: ").unwrap();

    filename

}

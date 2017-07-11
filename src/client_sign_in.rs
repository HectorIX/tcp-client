extern crate rpassword;


use std::io::{self};


pub fn sign_in() -> String {


    let mut full_request = "sign_up_state::Sign-in**".to_owned();

    println!("===============================\n\tSIGN IN\n\n" );
    let username = rpassword::prompt_response_stdout("username: ").unwrap();


    full_request.push_str(&username);
    full_request.push_str("--");

    let password = rpassword::prompt_password_stdout("password: ").unwrap();
    println!("\nPlease press enter...");
    full_request.push_str(&password);


    full_request.to_string()

}

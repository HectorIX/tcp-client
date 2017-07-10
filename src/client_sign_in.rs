use std::io::{self};


pub fn sign_in() -> String {

    let mut username = String::new();
    let mut password = String::new();
    let mut full_request = "sign_up_state::Sign-in**".to_owned();

    println!("===============================\n\tSIGN IN\n\n" );
    println!("username: ");
    io::stdin().read_line(&mut username).unwrap();

    let req_vector: Vec<&str> = username.split("\n").collect();
    let no_new_line = req_vector[0].to_string();

    full_request.push_str(&no_new_line);

    full_request.push_str("--");

    println!("password: ");
    io::stdin().read_line(&mut password).unwrap();

    println!("");
    full_request.push_str(&password);
    full_request.to_string()

}

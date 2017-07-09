use std::io::{self, Read};


pub fn sign_up() -> String {

    let mut username = String::new();
    let mut password = String::new();
    let mut full_request = "sign_up_state::Sign-up**".to_owned();


    println!("username: ");
    io::stdin().read_line(&mut username).unwrap();

    let mut req_vector: Vec<&str> = username.split("\n").collect();
    let mut no_new_line = req_vector[0].to_string();

    full_request.push_str(&no_new_line);

    full_request.push_str("--");
    println!("req = {:?}", full_request );
    println!("password: ");
    io::stdin().read_line(&mut password).unwrap();

    full_request.push_str(&password);

    println!("req = {:?}", full_request );
    full_request.to_string()
    //format!("sign_up_state::Sign-up**{0}--{1}", username, password.clone())
}

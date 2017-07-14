extern crate rpassword;


use std::io::{self};
use std::default::Default;


#[derive(Default)]
pub struct User {

    pub username : String,
    pub session_key : String,
    pub active : bool,
}


impl User {

    pub fn get_username(&self) -> &String {

        &self.username
    }

    pub fn get_session_key(&self) -> &String {

        &self.session_key
    }

    pub fn state(&mut self) -> &mut bool {

        &mut self.active
    }

}



pub fn sign_in() -> String {


    let mut full_request = "sign_in_state::Sign-in**".to_owned();

    println!("===============================\n\tSIGN IN\n\n" );
    let username = rpassword::prompt_response_stdout("username: ").unwrap();


    full_request.push_str(&username);
    full_request.push_str("--");

    let password = rpassword::prompt_password_stdout("password: ").unwrap();
    println!("\nPlease press enter...");
    full_request.push_str(&password);


    full_request.to_string()

}

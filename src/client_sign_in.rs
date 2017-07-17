extern crate rpassword;

use integrity;


pub fn sign_in() -> String {


    let mut full_request = "sign_in_state::Sign-in**".to_owned();

    println!("===============================\n\tSIGN IN\n\n" );
    let username = rpassword::prompt_response_stdout("username: ").unwrap();


    full_request.push_str(&username);
    full_request.push_str("--");

    let password = rpassword::prompt_password_stdout("password: ").unwrap();
    let hashed_password = integrity::sha3_512(password);

    full_request.push_str(&hashed_password);


    full_request.to_string()

}

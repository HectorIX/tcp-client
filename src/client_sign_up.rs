extern crate rpassword;


use integrity;



pub fn sign_up() -> String {


    let mut full_request = "sign_up_state::Sign-up**".to_owned();


    println!("===============================\n\tSIGN UP\n\n" );
    let username = rpassword::prompt_response_stdout("username: ").unwrap();


    full_request.push_str(&username);
    full_request.push_str("--");

    let password = rpassword::prompt_password_stdout("password: ").unwrap();
    let hashed_password = integrity::sha3_512(password);

    full_request.push_str(&hashed_password);


    full_request.to_string()

}

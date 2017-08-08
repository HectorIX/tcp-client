extern crate rpassword;

use std::fs;
use integrity;



pub fn sign_up() -> String {


    let mut full_request = "sign_up_state::Sign-up**".to_owned();


    println!("===============================\n\tSIGN UP\n\n" );
    let username = rpassword::prompt_response_stdout("username: ").unwrap();


    full_request.push_str(&username);
    full_request.push_str("--");

    let password = rpassword::prompt_password_stdout("password: ").unwrap();
    let confirm  = rpassword::prompt_password_stdout("Confirm password: ").unwrap();


    if password == confirm {

        let hashed_password = integrity::sha3_512(password);
        full_request.push_str(&hashed_password);

        let email = rpassword::prompt_response_stdout("email: ").unwrap();

        full_request.push_str("^^^^");
        full_request.push_str(&email);

        create_local_folders();

        full_request
    }
    else {

        full_request = "sign_up_state::Sign-up**UNCONFIRMED--UNCONFIRMED".to_string();
        full_request
    }

}



fn create_local_folders() {


    let download_dir = "download/".to_string();
    let upload_dir = "to_upload/".to_string();
    let local_data_dir = "local_data/".to_string();
    let raw_dir = "local_data/raw/".to_string();
    let decrypted_dir = "local_data/decrypted/".to_string();
    let encrypted_dir = "local_data/encrypted".to_string();
    let integrity_dir = "local_data/integrity".to_string();
    let key_manager_dir = "local_data/key_manager".to_string();



    match fs::create_dir(download_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }


    match fs::create_dir(upload_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }



    match fs::create_dir(local_data_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }


    match fs::create_dir(raw_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }


    match fs::create_dir(decrypted_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }


    match fs::create_dir(encrypted_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }


    match fs::create_dir(key_manager_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }


    match fs::create_dir(integrity_dir) {

        Err(e) => println!("Failed to create folder: {:?}", e.kind()),
        Ok(_)  => {},
    }

}

use menu;
use local_services;
use client_sign_up;
use client_sign_in;
use client_upload;

use bytes::{BytesMut};


pub fn request_constructor(req:String) -> String {

    let req_vector: Vec<&str> = req.split("\n").collect();
    let no_new_line = req_vector[0].to_string();


    match no_new_line.as_ref() {

        "help" => {
            menu::help_menu()
        },
        "local" => {
            menu::client_menu()
        },
        "net" => {
            menu::server_menu()
        },
        "Sign-up" => {
            client_sign_up::sign_up()
        },
        "Sign-in" => {
            client_sign_in::sign_in()
        },
        "Upload" => {
            client_upload::upload()
        },
        "Download" => {
            "download".to_string()
        },
        "Integrity" => {
            "integrity".to_string()
        },
        "AES-Encrypt" => {

            local_services::aes_encrypt();
            "".to_string()
        },
        "Caesar-Encrypt" => {

            local_services::caesar_encrypt();
            "".to_string()
        },
        "AES-Decrypt" => {

            local_services::aes_decrypt();
            "".to_string()
        },
        "Caesar-Decrypt" => {

            local_services::caesar_decrypt();
            "".to_string()
        },
        "exit" => {
            "informatic_state::exit**".to_string()
        },
        _ => {
            "\n\t+ Typpo(!!!), please try again...\n".to_string()
        },

    }

}


pub fn response_decomposer(response:String) -> BytesMut {

    if response.contains("session_key") {
        BytesMut::from("success\n")
    } else {
        BytesMut::from("not_success\n")
    }
}

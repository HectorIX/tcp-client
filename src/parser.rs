use menu;
use local_services;
use client_sign_up;
use client_sign_in;



pub fn request_constructor(req:String) -> String {

    let req_vector: Vec<&str> = req.split("\n").collect();
    let no_new_line = req_vector[0].to_string();


    match no_new_line.as_ref() {

        "help" => {
            "informatic_state::help**".to_string()
        },
        "start" => {
            "informatic_state::start**".to_string()
        },
        "local" => {
            menu::client_menu()
        },
        "net" => {
            "informatic_state::menu**".to_string()
        },
        "Sign-up" => {
            client_sign_up::sign_up()
        },
        "Sign-in" => {
            client_sign_in::sign_in()
        },
        "Upload" => {
            "upload".to_string()
        },
        "Download" => {
            "download".to_string()
        },
        "Integrity" => {
            "integrity".to_string()
        },
        "AES-Encrypt" => {
            "encrypt".to_string()
        },
        "Caesar-Encrypt" => {

            local_services::caesar_encrypt();
            "".to_string()
        },
        "AES-Decrypt" => {
            "decrypt".to_string()
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

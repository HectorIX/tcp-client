use menu;
use client_sign_up;


pub fn request_constructor(req:String) -> String {

    let req_vector: Vec<&str> = req.split("\n").collect();
    let no_new_line = req_vector[0].to_string();

    //println!("no_new_line = {:?}", no_new_line.clone() );

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
        "sign_in" => {
            "sign_in".to_string()
        },
        "upload" => {
            "upload".to_string()
        },
        "download" => {
            "download".to_string()
        },
        "integrity" => {
            "integrity".to_string()
        },
        "encrypt" => {
            "encrypt".to_string()
        },
        "decrypt" => {
            "decrypt".to_string()
        },
        "informatic_state::exit" => {
            "exit".to_string()
        },
        _ => {
            "\n\t+ Typpo(!!!), please try again...\n".to_string()
        },

    }

}

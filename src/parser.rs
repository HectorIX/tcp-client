use menu;
use user;
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

            if !user::get_user_status() {

                client_sign_up::sign_up()
            }
            else {

                "sign_up_state::Sign-up**".to_string()
            }

        },
        "Sign-in" => {

            if !user::get_user_status() {

                client_sign_in::sign_in()
            }
            else {

                "sign_in_state::Sign-in**".to_string()
            }

        },
        "Upload" => {

            if user::get_user_status() {

                client_upload::upload()
            }
            else {

                "upload_state::Upload**".to_string()
            }
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


pub fn response_decomposer(server_response:String) -> BytesMut {


    let (the_state, response) = response_splitter(server_response);
    let (status, data) = status_splitter(response);


    match the_state.as_ref() {

        "sign_in_state" => {

            if status == "OK".to_string() {

                let (session_key, username) = extract_session_key(data);

                let mut name = username;
                let mut key  = session_key;

                name.pop(); // no '\n'
                key.pop();

                user::set_username(name);
                user::set_session_key(key);
                user::set_user_status(true);

                let welcome_message = format!("\n\t==========    Welcome {}!    ==========\n\n\n", user::get_username());

                BytesMut::from(welcome_message)

            }
            else if status == "NOT_Mactching".to_string() {

                BytesMut::from("\n\n\t*** Either your username or password are incorrect.\n\t    Please try again...\n")
            }
            else if status == "ALREADY_Sign_in".to_string() {

                BytesMut::from("\n\t You are already Loged-In!\n")
            }
            else {

                BytesMut::from("No such state!\n")
            }

        },
        "sign_up_state" => {

            if status == "OK".to_string() {

                BytesMut::from("Congradulations! You signed up successfully!\n")
            }
            else if status == "Dublicate".to_string() {

                BytesMut::from("\n\t ** Username already exists!\n")
            }
            else if status == "Unauthorised".to_string() {

                BytesMut::from("\n\t ** You are not authorised to Sign-up while Loged-In!\n")
            }
            else {

                BytesMut::from("No such state\n" )
            }
        },
        "upload_state" => {

            if status == "OK".to_string() {

                BytesMut::from("Upload completed!\n")
            }
            else if status == "Failed".to_string() {

                BytesMut::from("Service declined! You are not authorized user... Please login!\n")
            }
            else {

                BytesMut::from("No such state!\n")
            }

        },
        "download_state" => {
            BytesMut::from("do")
        },
        _ => {
            BytesMut::from("No such state!" )  // unreachable
        },
    }


}


fn response_splitter( server_response:String ) -> (String, String) {

    let response_vector: Vec<&str> = server_response.split("::").collect();
    let (the_state, request) = (response_vector[0].to_string(), response_vector[1].to_string());

    (the_state, request)
}


fn status_splitter( response_service:String ) -> (String, String) {

    let response_vector: Vec<&str> = response_service.split("**").collect();
    let (status, data) = (response_vector[0].to_string(), response_vector[1].to_string());

    (status, data)
}



fn extract_session_key(data:String) -> (String, String)  {

    let data_vector: Vec<&str> = data.split("--").collect();
    let (session_key, rest_data) = (data_vector[0].to_owned(), data_vector[1].to_owned());

    (session_key, rest_data)
}

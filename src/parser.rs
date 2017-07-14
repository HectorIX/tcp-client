use menu;
use local_services;
use client_sign_up;
use client_sign_in;
use client_upload;

use std::default::Default;
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


pub fn response_decomposer(server_response:String) -> BytesMut {

    let (the_state, response) = response_splitter(server_response);
    let (status, data) = status_splitter(response);


    match the_state.as_ref() {

        "sign_in_state" => {

            if status == "OK".to_string() {

                let (session_key, username) = extract_session_key(data);
                let mut user = client_sign_in::User::default();

                user.username = username;
                user.session_key = session_key;
                user.active = true;

                //println!("Username = {}", user.get_username() );
                //println!("session_key = {}", user.get_session_key() );

                BytesMut::from("\n\t==========    Welcome!    ==========\n\n\n")

            }
            else {
                BytesMut::from("\n\n\t*** Either your username or password are incorrect.\n\t    Please try again...\n")
            }

        },
        "sign_up_state" => {
            BytesMut::from("si")
        },
        "upload_state" => {
            BytesMut::from("up")
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

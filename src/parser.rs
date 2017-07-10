use menu;
use interface;
use file_io;
use client_sign_up;
use client_sign_in;
use caesar_cipher;
use aes_256;


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

            let mut path_raw = "local_data/raw/".to_string();
            let mut path_encr = "local_data/encrypted/".to_string();


            let filename = interface::read_filename();
            let filename_splitted:Vec<&str> = filename.split("\n").collect();

            path_raw.push_str(&filename_splitted[0].to_string());
            path_encr.push_str(&filename_splitted[0].to_string());

            let plaintext = file_io::read_file(path_raw.to_string());

            let encryption_key = interface::read_integer();
            let encrypted_message = caesar_cipher::encrypt_caesar(plaintext,encryption_key);

            file_io::write_file(path_encr,encrypted_message);

            println!("File {} successfully encrypted!", filename_splitted[0].to_string());

            "".to_string()
        },
        "AES-Decrypt" => {
            "decrypt".to_string()
        },
        "Caesar-Decrypt" => {
            "caesar".to_string()
        },
        "exit" => {
            "informatic_state::exit**".to_string()
        },
        _ => {
            "\n\t+ Typpo(!!!), please try again...\n".to_string()
        },

    }

}

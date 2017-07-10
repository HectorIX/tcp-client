use interface;
use file_io;
use caesar_cipher;
use aes_256;


pub fn caesar_encrypt() {

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

}



pub fn caesar_decrypt() {

    let mut path_encr = "local_data/encrypted/".to_string();
    let mut path_decr = "local_data/decrypted/".to_string();

    let filename = interface::read_filename();
    let filename_splitted:Vec<&str> = filename.split("\n").collect();

    path_encr.push_str(&filename_splitted[0].to_string());
    path_decr.push_str(&filename_splitted[0].to_string());

    let encrypted_message = file_io::read_file(path_encr.to_string());

    let encryption_key = interface::read_integer();
    let decrypted_message = caesar_cipher::decrypt_caesar(encrypted_message,encryption_key);

    file_io::write_file(path_decr,decrypted_message);

    println!("File {} successfully decrypted!", filename_splitted[0].to_string());


}

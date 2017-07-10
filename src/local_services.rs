extern crate rand;

use self::rand::{ Rng, OsRng };

use std;
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


pub fn aes_encrypt() {


    // Key and IV for the AES-256 (de)encryption algorithm.
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    // Local variables.
    let mut encr_message: String = "".to_string();
    let mut rng = OsRng::new().ok().unwrap();

    // Create a random key and a random IV.
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);



    let mut path_raw = "local_data/raw/".to_string();
    let mut path_encr = "local_data/encrypted/".to_string();

    let filename = interface::read_filename();
    let filename_splitted:Vec<&str> = filename.split("\n").collect();

    path_raw.push_str(&filename_splitted[0].to_string());
    path_encr.push_str(&filename_splitted[0].to_string());

    let plaintext = file_io::read_file(path_raw.to_string());


    // Encrypt the message using AES-256 encryption algorithm.
    let encrypted_message = aes_256::aes256_encrypt( &plaintext.into_bytes(),
                                                     &key,
                                                     &iv).ok().unwrap();

    let mut encrypted_string = "".to_string();

    // Convert the message from byte stream to String.
    for byte in encrypted_message {
        encrypted_string.push(std::char::from_u32(byte as u32).unwrap());
    }

    // Store the message to the "aesOutput.txt"
    file_io::write_file(path_encr, encrypted_string);
    println!("Your message is secure!");

}

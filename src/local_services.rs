extern crate rand;
extern crate rpassword;


use std;
use interface;
use file_io;
use caesar_cipher;
use aes_256;
use generate_password;
use integrity;




pub fn caesar_encrypt() -> String {

    let mut path_raw = "local_data/raw/".to_string();
    let mut path_encr = "local_data/encrypted/".to_string();

    let filename = interface::read_filename();

    path_raw.push_str(&filename);
    path_encr.push_str(&filename);

    let plaintext = file_io::read_file(path_raw.to_string());

    let encryption_key = interface::read_integer();
    let encrypted_message = caesar_cipher::encrypt_caesar(plaintext, encryption_key);

    let operation = file_io::write_file(path_encr, encrypted_message);



    if !operation.starts_with("**Failed") {

        println!("\n\t*** File successfully encrypted!");
    }
    else {

        println!("\n\t*** Failed to create/write: {}", filename );
    }


    "".to_string()

}



pub fn caesar_decrypt() -> String {

    let mut path_encr = "local_data/encrypted/".to_string();
    let mut path_decr = "local_data/decrypted/".to_string();

    let filename = interface::read_filename();

    path_encr.push_str(&filename);
    path_decr.push_str(&filename);

    let encrypted_message = file_io::read_file(path_encr.to_string());

    let encryption_key = interface::read_integer();
    let decrypted_message = caesar_cipher::decrypt_caesar(encrypted_message,encryption_key);

    let operation = file_io::write_file(path_decr,decrypted_message);



    if !operation.starts_with("**Failed") {

        println!("\n\t*** File successfully dencrypted!");
    }
    else {

        println!("\n\t*** Failed to create/write: {}", filename );
    }

    "".to_string()
}


pub fn aes_encrypt() -> String {


    let path_symmetric_key = "local_data/key_manager/symmetric_key.txt".to_string();
    let path_iv = "local_data/key_manager/initialize_vector.txt".to_string();


    let mut path_raw = "local_data/raw/".to_string();
    let mut path_encr = "local_data/encrypted/".to_string();

    let filename = interface::read_filename();


    path_raw.push_str(&filename);
    path_encr.push_str(&filename);


    // Generate random password and initialize vector.
    let symmetric_key = generate_password::random_password(32);
    let initialize_vector = generate_password::random_password(16);


    // Key and IV for the AES-256 (de)encryption algorithm as bytes.
    let key = symmetric_key.as_bytes();
    let iv  = initialize_vector.as_bytes();


    // Read the context of the file.
    let plaintext = file_io::read_file(path_raw.to_string());


    if !plaintext.starts_with("**Failed") {

        // Encrypt the message using AES-256 encryption algorithm.
        let encrypted_message = aes_256::aes256_encrypt( &plaintext.into_bytes(),
                                                         &key,
                                                         &iv).ok().unwrap();

        let mut encrypted_string = "".to_string();

        // Convert the message from byte stream to String.
        for byte in encrypted_message {
            encrypted_string.push(std::char::from_u32(byte as u32).unwrap());
        }


        // Store the encrypted message to the file path: "path_encr".
        let op_encr = file_io::write_file(path_encr, encrypted_string);

        // Store the random generated symmetric key to the file path: "path_symmetric_key".
        let op_key = file_io::write_file(path_symmetric_key, symmetric_key.clone());

        //Store the random generated iv to the file path: "path_iv".
        let op_iv = file_io::write_file(path_iv, initialize_vector.clone());


           if !op_encr.starts_with("**Failed") &
              !op_key.starts_with("**Failed") &
              !op_iv.starts_with("**Failed") {

               println!("\n\t*** Your message is secure!");

           }
           else {

               println!("\n\t*** Failed to store the file and its keys to the appropriate folders." );
           }

    }
    else {

        println!("\n\t*** Failed to open/read file: {}", filename);
    }


    "".to_string()
}


pub fn aes_decrypt() -> String {

    let symmetric_key = rpassword::prompt_password_stdout("Symmetric key: ").unwrap();
    let initialize_vector = rpassword::prompt_password_stdout("Initialize Vector: ").unwrap();

    // Key and IV for the AES-256 (de)encryption algorithm into bytes.
    let key = symmetric_key.as_bytes();
    let iv = initialize_vector.as_bytes();


    let mut path_encr = "local_data/encrypted/".to_string();
    let mut path_decr = "local_data/decrypted/".to_string();

    let filename = interface::read_filename();

    path_encr.push_str(&filename);
    path_decr.push_str(&filename);

    let encrypted_message = file_io::read_file(path_encr.to_string());



    if !encrypted_message.starts_with("**Failed") {


        let mut encr_message_as_bytes: Vec<u8> = Vec::new();

        // Convert this message to a byte stream.
        for character in encrypted_message.chars() {
            encr_message_as_bytes.push(character as u8);
        }

        // Encrypt the message using AES-256 encryption algorithm.
        let decr_message_as_bytes = aes_256::aes256_decrypt( &encr_message_as_bytes[..],
                                                             &key,
                                                             &iv);

        let mut decr_message = "".to_string();


        // Convert the message into String.
        for byte in decr_message_as_bytes.unwrap() {
                decr_message.push(byte as char);
        }


        let op_decr = file_io::write_file(path_decr, decr_message);

        if !op_decr.starts_with("**Failed") {

            println!("\n\t*** Your message decrypted!");
        }
        else {

            println!("\n\t*** Failed to create/write the encrypted file." );
        }

    }
    else {

        println!("\n\t*** Failed to open/read filen: {}", filename );
    }


    "".to_string()
}






pub fn integrity() -> String {

    let mut path_raw = "local_data/raw/".to_string();
    let mut data = "<**>\n".to_string();

    let path_integrity = "local_data/integrity/hash_values.txt".to_string();


    let filename = interface::read_filename();

    path_raw.push_str(&filename);

    let file_context = file_io::read_file(path_raw);


    if !file_context.starts_with("**Failed") {

        let whirlpool = integrity::whirlpool(file_context.clone());
        let sha3 = integrity::sha3_512(file_context.clone());

        data.push_str("File:: ");
        data.push_str(&filename);
        data.push_str("\nsha3_512:: ");
        data.push_str(&sha3);
        data.push_str("\nwhirpool:: ");
        data.push_str(&whirlpool);
        data.push_str("\n");

        file_io::append(path_integrity, data);

        println!("\n\t*** Hash values computed!\n");

    }
    else {

        println!("\n\t*** Operation failed. There is no file: {}\n", filename );
    }



    "".to_string()
}

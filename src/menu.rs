
pub fn welcome_menu() {


    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n");
    println!("\t\t WELCOME   TO   THE   RUST  ENCRYPTOR\n");
    println!("\t+ Type help for help..." );
    println!("\t+ Type local to get the menu for client side services.");
    println!("\t+ Type net to get the menu for server side services.");
    println!("\t+ Type exit to terminate the program.");
    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n")
}



pub fn client_menu() {

    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n");
    println!("\t 1. Store your plaintext message in a file.");
    println!("\t 2. Encrypt the plaintext message using Caesar Cipher.");
    println!("\t 3. Decrypt the ciphertext [Caesar Cipher decryptor].");
    println!("\t 4. Encrypt the plaintext message using AES-256.");
    println!("\t 5. Decrypt the ciphertext [AES-256 decryptor].");
    println!("\t 6. Calculate the SHA-256 signature of a file.");
    println!("\t 7. Exit Program.");
    println!("\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n")
}

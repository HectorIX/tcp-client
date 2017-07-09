
pub fn welcome_menu() {


    println!("\n=================================================================\n");
    println!("\t\t WELCOME   TO   THE   RUST  ENCRYPTOR\n");
    println!("\t<+> Type help for help..." );
    println!("\t<+> Type local to get the menu for client side services.");
    println!("\t<+> Type net to get the menu for server side services.");
    println!("\t<+> Type exit to terminate the program.");
    println!("\n=================================================================\n")
}



pub fn client_menu() -> String {

    println!("\n===========================================================================");
    println!("\t\t\tCLIENT SIDE MENU\n");
    println!("\t <+> To encrypt a file using Caesar Cipher type: Caesar-Encrypt");
    println!("\t <+> To encrypt a file using AES-256 type: AES-Encrypt");
    println!("\t <+> To decrypt a file [encrypted by Caesar Cipher] type: Caesar-Decrypt");
    println!("\t <+> To decrypt a file [encrypted by AES-256] type: AES-Decrypt");
    println!("\t <+> Type exit to terminate the program.");
    println!("\n===========================================================================\n");

    "".to_string()
}

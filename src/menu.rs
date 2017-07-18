
pub fn welcome_menu() {


    println!("\n=================================================================\n");
    println!("\t\t WELCOME   TO   THE   RUST  ENCRYPTOR\n");
    println!("\t<+> Type help for help..." );
    println!("\t<+> Type local to get the menu for client side services.");
    println!("\t<+> Type net to get the menu for server side services.");
    println!("\t<+> Type exit to terminate the program.");
    println!("\n=================================================================\n\n")
}



pub fn help_menu() -> String {

    println!("\n     ====================   HELP MENU   ====================\n
    \t<+> To use our clinet side services type: local
    \t<+> To use our server side services type: net\n");

    "".to_string()
}


pub fn client_menu() -> String {

    println!("\n=========================    CLIENT SIDE MENU    ==========================\n");

    println!("    <+> To encrypt a file using Caesar Cipher type: Caesar-Encrypt");
    println!("    <+> To encrypt a file using AES-256 type: AES-Encrypt");
    println!("    <+> To decrypt a file [encrypted by Caesar Cipher] type: Caesar-Decrypt");
    println!("    <+> To decrypt a file [encrypted by AES-256] type: AES-Decrypt");
    println!("    <+> To compute the hash value of a file type: Integrity");

    "".to_string()
}




pub fn server_menu() -> String {


    let horizontal_line = "\n=========================    SERVER MENU    =======================\n\n";
    let sign_out  = "\t <:> To sign-out type: Disconnect\n";
    let sign_in   = "\t <:> To sign-in type: Sign-in\n";
    let sign_up   = "\t <:> To sign-up type: Sign-up\n";
    let connect   = "\t <:> To connect to the server type: Connect\n";
    let upload    = "\t <:> To upload an encryped file type: Upload\n";
    let download  = "\t <:> To download a file of yours type: Download\n";
    let tip       = "\n\t [~Tip~] :: Type Connect first!\n";



    println!("{} {} {} {} {} {} {} {}",
                                    horizontal_line,
                                    sign_up,
                                    sign_in,
                                    sign_out,
                                    connect,
                                    upload,
                                    download,
                                    tip
    );

    "".to_string()

}

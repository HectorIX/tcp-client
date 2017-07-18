
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;
extern crate rpassword;



mod codec;
mod menu;
mod user;
mod parser;
mod file_io;
mod interface;
mod local_services;
mod client_sign_up;
mod client_sign_in;
mod client_upload;
mod client_download;
mod caesar_cipher;
mod aes_256;
mod generate_password;
mod integrity;

use std::env;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::thread;
use std::str;


use futures::sync::mpsc;
use futures::{ Future, Stream};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;







fn main() {


    menu::welcome_menu();

    service_loop();


}







fn service_loop() {


        loop {

            println!("");
            let instruction = rpassword::prompt_response_stdout(">> ").unwrap();


            match instruction.as_ref() {

                "help" => {

                    menu::help_menu();
                },
                "local" => {

                    menu::client_menu();
                },
                "net" => {

                    menu::server_menu();
                },
                "Connect" => {

                    println!("\n\t ** Connected!\n\n");
                    connect();

                },
                "exit" => {

                    break;
                },
                "Sign-up"  | "Sign-in" |
                "Download" | "Upload"  => {

                    println!("\n\t+ You have to get connected!\n");
                    println!("\t[~Tip~] :: To connect type: Connect\n");
                },
                "Disconnect" => {

                    println!("\t+ You are not connected...");
                },
                "Integrity" => {

                    local_services::integrity();
                },
                "AES-Encrypt" => {

                    local_services::aes_encrypt();
                },
                "Caesar-Encrypt" => {

                    local_services::caesar_encrypt();
                },
                "AES-Decrypt" => {

                    local_services::aes_decrypt();
                },
                "Caesar-Decrypt" => {

                    local_services::caesar_decrypt();
                },
                _ => {

                    println!("\n\t+ Typpo(!!!), please try again...\n");
                },
            }
        }

}


fn connect() {



    // Parse what address we're going to connect to
    let addr = env::args().nth(1).unwrap_or_else(|| {
        panic!("this program requires at least one argument")
    });
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Create the event loop and initiate the connection to the remote server
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpStream::connect(&addr, &handle);

        
        let (stdin_tx, stdin_rx) = mpsc::channel(0);
        thread::spawn(|| codec::read_stdin(stdin_tx));
        let stdin_rx = stdin_rx.map_err(|_| panic!()); // errors not possible on rx


        let mut stdout = io::stdout();

        let client = tcp.and_then(|stream| {

            let (sink, stream) = stream.framed(codec::Bytes).split();
            let send_stdin = stdin_rx.forward(sink);
            let write_stdout = stream.for_each(move |buf| {

                stdout.write_all(&buf)
            });

            send_stdin.map(|_| ())
                      .select(write_stdout.map(|_| ()))
                      .then(|_| Ok(()))
        });

        // And now that we've got our client, we execute it in the event loop!
        core.run(client).unwrap();

}

//! A simple example of hooking up stdin/stdout to a TCP stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.

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

        // Right now Tokio doesn't support a handle to stdin running on the event
        // loop, so we farm out that work to a separate thread. This thread will
        // read data from stdin and then send it to the event loop over a standard
        // futures channel.
        let (stdin_tx, stdin_rx) = mpsc::channel(0);
        thread::spawn(|| codec::read_stdin(stdin_tx));
        let stdin_rx = stdin_rx.map_err(|_| panic!()); // errors not possible on rx

        // After the TCP connection has been established, we set up our client to
        // start forwarding data.
        //
        // First we use the `Io::framed` method with a simple implementation of a
        // `Codec` (listed below) that just ships bytes around. We then split that
        // in two to work with the stream and sink separately.
        //
        // Half of the work we're going to do is to take all data we receive on
        // stdin (`stdin_rx`) and send that along the TCP stream (`sink`). The
        // second half is to take all the data we receive (`stream`) and then write
        // that to stdout. Currently we just write to stdout in a synchronous
        // fashion.
        //
        // Finally we set the client to terminate once either half of this work
        // finishes. If we don't have any more data to read or we won't receive any
        // more work from the remote then we can exit.
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

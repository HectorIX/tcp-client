use bytes::{BufMut, BytesMut};
use futures::sync::mpsc;
use futures::{Sink, Future};
use tokio_io::codec::{Encoder, Decoder};


use std::io::{self, Read};
use std::str;
use parser;





/// A simple `Codec` implementation that just ships bytes around.
///
/// This type is used for "framing" a TCP stream of bytes but it's really just a
/// convenient method for us to work with streams/sinks for now. This'll just
/// take any data read and interpret it as a "frame" and conversely just shove
/// data into the output location without looking at it.
pub struct Bytes;

impl Decoder for Bytes {

    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {

        if buf.len() > 0 {

            let end = buf.len();
            let input = buf.split_to(end);

            let response = str::from_utf8(&input).unwrap();
            let output =  parser::response_decomposer(response.to_string());

            Ok(Some(output))

        } else {

            Ok(None)
        }

    }


    fn decode_eof(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        self.decode(buf)
    }
}

impl Encoder for Bytes {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.put(&data[..]);
        Ok(())
    }
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
pub fn read_stdin(mut tx: mpsc::Sender<Vec<u8>>) {


    let mut stdin = io::stdin();


    loop {

        let mut buf = vec![0; 1024];

        let n = match stdin.read(&mut buf) {
            Err(_) |
            Ok(0) => break,
            Ok(n) => n,
        };

        buf.truncate(n);

        let full_request = translate(buf);



        if full_request == "exit".to_string().into_bytes() {

            break;
        }
        else {

            tx = tx.send(full_request.clone()).wait().unwrap();
        }

    }
}



fn translate(buffer:Vec<u8>) -> Vec<u8> {

    let readble_form = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    };

    let client_request = parser::request_constructor(readble_form.to_string());

    client_request.to_string().into_bytes()
}

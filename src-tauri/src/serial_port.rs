// use base64::encode;
use futures::stream::StreamExt;
use std::{io, str};
use tauri::Window;
// use tokio::io::AsyncWriteExt;
use tokio_util::codec::{Decoder, Encoder};

use bytes::BytesMut;
use tokio_serial::SerialPortBuilderExt;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

const DEFAULT_TTY: &str = "COM2";

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'|');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[tokio::main]
pub async fn start_serial(window: Window) -> tokio_serial::Result<()> {
    let port = tokio_serial::new(DEFAULT_TTY, 9600)
        .open_native_async()
        .expect("unable to open serial port");

    // let message = encode("This is a test message");

    // port.write_all(message.as_bytes())
    //     .await
    //     .expect("unable to write test message");

    let mut reader = LineCodec.framed(port);

    println!("aa");
    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
        window
            .emit(
                "event-name",
                Payload {
                    message: line.into(),
                },
            )
            .unwrap();
    }
    Ok(())
}

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
#[macro_use] extern crate slice_as_array;

use std::io;


fn get(db: Arc<Mutex<HashMap<String, String>>>) {
    println!("the get is working");
}

fn insert(db: Arc<Mutex<HashMap<String, String>>>) {
    println!("the insert is working");
}

fn delete(db: Arc<Mutex<HashMap<String, String>>>) {
    println!("the delete is working");
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let db: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let db = db.clone();
        // process_socket(socket, db.clone()).await;
        tokio::spawn(async move {
            let mut buf: [u8 ; 1024 ]= [0; 1024];
            // In a loop, read data from the socket and write the data back.
            loop {
                println!("loop started");
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                let header_buf = slice_as_array!(&buf[0 .. 4], [u8; 4]).expect("bad hash length");
                // let slice = buf[0 .. 4].clone_into([u8 ; 4]).unwrap();
                let header = i32::from_be_bytes(*header_buf);
                match header {
                    1 => insert(db.clone()),
                    2 => get(db.clone()),
                    3 => delete(db.clone()),
                    _ => panic!("operation not allowed")
                }
                println!("{:?}", header);
                // println!("{}", &buf[0] as String);

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
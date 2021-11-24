use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
#[macro_use] extern crate slice_as_array;

use std::io;
use std::str;


#[derive(Debug)]
enum ValueStore {
    String(String),
    Int(i32)
}


fn get(db: Arc<Mutex<HashMap<String, ValueStore>>>) {
    println!("the get is working");
}

fn insert(db: Arc<Mutex<HashMap<String, ValueStore>>>, key: String, value: ValueStore) {
    println!("the insert is working");
    let mut database = db.lock().unwrap();
    database.insert(key, value);
    println!("{:?}", database);
}

fn delete(db: Arc<Mutex<HashMap<String, ValueStore>>>) {
    println!("the delete is working");
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let db: Arc<Mutex<HashMap<String, ValueStore>>> = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {

            let mut buf: [u8 ; 1024 ]= [0; 1024];

            let n = match socket.read(&mut buf).await {

                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let operation_buf = slice_as_array!(&buf[0 .. 4], [u8; 4]).expect("failed to extract operation");
            let operation = i32::from_be_bytes(*operation_buf);

            let data_type_buf = slice_as_array!(&buf[4 .. 8], [u8; 4]).expect("failed to extract operation");
            let data_type = i32::from_be_bytes(*data_type_buf);

            let key_buf = slice_as_array!(&buf[8 .. 28], [u8; 20]).expect("could not extract key");
            let key = str::from_utf8(key_buf).unwrap().to_string();

            let value: ValueStore;

            match data_type {
                1 => {
                    let value_buf = slice_as_array!(&buf[28 .. 428], [u8; 400]).expect("could not extract value");
                    value = ValueStore::String(str::from_utf8(value_buf).unwrap().to_string());
                },
                2 => {
                    let value_buf = slice_as_array!(&buf[28 .. 32], [u8; 4]).expect("could not extract value");
                    value = ValueStore::Int(i32::from_be_bytes(*value_buf));
                },
                _ => panic!("data type not allowed")
            }

            match operation {
                1 => { insert(db.clone(), key, value) },
                2 => { get(db.clone()) },
                3 => { delete(db.clone()) },
                _ => panic!("operation not allowed")
            }

            if let Err(e) = socket.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return;
            }
        });
    }
}

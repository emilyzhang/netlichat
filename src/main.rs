use std::{sync::Arc, time::Duration};

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    spawn,
    sync::{mpsc, Mutex},
    time::sleep,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7777").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let (tx, mut rx) = mpsc::channel(100);
        let _ = tx.send(25).await;
        let stream = Arc::new(Mutex::new(stream)); // don't have to lifetimes? arc handles it for you?
        spawn(handle_stream(stream.clone()));
        spawn(handle_stream(stream));
        // when in doubt, use reference counting! Arc type
    }
}

async fn handle_stream(stream: Arc<Mutex<TcpStream>>) {
    loop {
        if let Err(e) = stream.lock().await.write_all(b"hello!\n").await {
            println!("Client went away: {}", e);
        };
        sleep(Duration::from_secs(1)).await
    }
}
// use std::{
//     io::Write,
//     net::{TcpListener, TcpStream},
//     thread::{sleep, spawn},
//     time::Duration,
// };

// // we're taking ownership of the stream, and then it gets dropped
// fn handle_client(mut stream: TcpStream) {
//     // ...
//     loop {
//         if let Err(e) = stream.write_all(b"HELLO WORLD\n") {
//             println!("The client went away! {}", e);
//             return;
//         } // use byte string instead of separately creating characters
//         sleep(Duration::from_secs(1))
//     }
// }

// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:7777")?;

//     // accept connections and process them serially
//     for stream in listener.incoming() {
//         // in JS (arg1, arg2) => {}
//         // in go, it looks like a regular function
//         // in rust, it's different
//         spawn(|| {
//             handle_client(stream.unwrap());
//         });
//         // handle_client(stream?);
//     }
//     Ok(())
// }

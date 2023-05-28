use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;

pub async fn run() {
    let host = "localhost:42069";

    let srv = net::TcpListener::bind(host).await.unwrap();

    loop {
        let (mut sock, _) = srv.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let n = sock.read(&mut buf).await.unwrap();
            sock.write_all(&buf[0..n]).await.unwrap();
            let data = str::from_utf8(&buf[0..n]).unwrap();
            println!("{:?}", data);
            sock.shutdown().await.unwrap();
        });
    }
}

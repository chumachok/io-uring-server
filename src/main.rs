use std::{
    io::self,
    net::{TcpListener, TcpStream},
};

use futures::executor::{block_on};

async fn handle_request(ring: &rio::Rio, stream: &TcpStream) -> io::Result<()> {
    let mut buf = vec![0_u8; 1024];
    loop {
        let n = ring.read_at(stream, &buf, 0).await?;
        buf[n] = b'\n';
        ring.write_at(stream, &buf, 0).await?;
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 3003))?;
    let ring = rio::new()?;

    block_on(async {
        loop {
            let stream = ring.accept(&listener).await.unwrap();
            handle_request(&ring, &stream).await;
        }
    })
}
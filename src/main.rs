use std::{
    io::self,
    thread,
    net::{TcpListener, TcpStream},
};

use futures::executor::{block_on};

const NUM_WORKERS: i32 = 5;

async fn handle_request(ring: &rio::Rio, stream: &TcpStream) -> io::Result<()> {
    let mut buf = vec![0_u8; 1024];
    loop {
        let n = ring.read_at(stream, &buf, 0).await?;
        buf[n] = b'\n';
        ring.write_at(stream, &buf, 0).await?;
    }
}

async fn spawn_server(listener: &TcpListener) -> io::Result<()> {
    let ring = rio::new().unwrap();
    loop {
        let stream = ring.accept(&listener).await.unwrap();
        handle_request(&ring, &stream).await?;
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 3003)).unwrap();
    let ring = rio::new().unwrap();

    for _ in 0..NUM_WORKERS {
        let lc = listener.try_clone().unwrap();
        thread::spawn(move || {
            block_on(async {
                spawn_server(&lc).await.unwrap_or_else(|err| println!("{:?}", err));
            });
        });
    }

    block_on(async {
        loop {
            let stream = ring.accept(&listener).await.unwrap();
            handle_request(&ring, &stream).await.unwrap_or_else(|err| println!("{:?}", err));
        }
    });

    Ok(())
}
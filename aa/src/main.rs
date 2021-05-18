use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use std::error::Error;

async fn do_stuff(mut sock: TcpStream, addr: SocketAddr) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    loop {
        let mut bytes = [0u8; 1024];

        let mut sz = sock.read(&mut bytes).await?;

        if sz == 0 {
            break;
        }

        let str = format!(" recevived from:{}", addr);

        str.as_bytes().iter().for_each(|c| {
            sz += 1;
            if sz < bytes.len() {
                bytes[sz] = *c;
            }
        });

        count += sock.write(&bytes[..sz]).await?;
    }

    Ok(count)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let ltn = TcpListener::bind("0.0.0.0:9999").await?;

    loop {
        let (sock, addr) = ltn.accept().await?;

        tokio::spawn(async move {
            let wrote = do_stuff(sock, addr)
                .await
                .map_err(|err| println!("got err {}", err))
                .unwrap();

            println!("wrote {} to {}", wrote, addr);
        });
    }

    Ok(())
}

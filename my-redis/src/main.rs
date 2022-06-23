use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listner.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        print!("GOT: {:?}", frame);

        let resoponse = Frame::Error("uniplemented".to_string());
        connection.write_frame(&resoponse).await.unwrap();
    }
}
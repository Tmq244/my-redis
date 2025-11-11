use tokio::net::{TcpListener,TcpStream};
use mini_redis::{Connection,Frame};

#[tokio::main]
async fn main(){
    //bind the listener to the address
    let listener =TcpListener::bind("127.0.0.1:6389").await.unwrap();

    loop{
        let (socket,_)=listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream){
    //the result of Connection is frames (command of redis + data) 
    let mut connection=Connection::new(socket);

    if let Some(frame)=connection.read_frame().await.unwrap(){
        println!("GOT: {:?}",frame);

        let response =Frame::Error("unimplement".to_string());
        connection.write_frame(&response).await.unwrap();
    }


}
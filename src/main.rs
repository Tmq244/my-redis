use tokio::net::{TcpListener,TcpStream};
use mini_redis::{Connection,Frame};
use std::collections::HashMap;
use mini_redis::Command::{Set,Get,self};
use bytes::Bytes;
use std::sync::{Arc,Mutex};

type Db=Arc<Mutex<HashMap<String,Bytes>>>;


#[tokio::main]
async fn main(){
    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener =TcpListener::bind("127.0.0.1:6389").await.unwrap();

    println!("Listener");

    let db=Arc::new(Mutex::new(HashMap::new()));


    loop{
        let (socket,_)=listener.accept().await.unwrap();

        let db=db.clone();
        println!("Accept");
        //为每一条连接都生成一个新的任务，并将socket的所有权转移
        tokio::spawn( async move {
            process(socket,db).await;
        });
    }
}

async fn process(socket: TcpStream,db:Db){


    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    let mut connection=Connection::new(socket);

    // 使用 `read_frame` 方法从连接获取一个数据帧：一条redis命令 + 相应的数据
    while let Some(frame)=connection.read_frame().await.unwrap(){
        let response=match Command::from_frame(frame).unwrap(){
            Set(cmd)=>{
                let mut db=db.lock().unwrap();
                db.insert(cmd.key().to_string(),cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd)=>{
                let db=db.lock().unwrap();
                if let Some(value)=db.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                }else{
                    Frame::Null
                }
            },
            cmd=>panic!("unimplemented {:?}",cmd)

        };

        connection.write_frame(&response).await.unwrap();
    }


}
use mini_redis::{client,Result};

#[tokio::main]
async fn main() -> Result<()> {
    // connect with mini-redis
    let mut client=client::connect("127.0.0.1:6389").await?;

    client.set("hello","world".into()).await?;

    let result=client.get("hello").await?;

    println!("The value of hello is {:?}",result);

    Ok(())

}

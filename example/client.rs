use mini_redis::client;

#[tokio::main]
async fn main() {
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    let mut client = client::connect("127.0.0.1:6380").await.unwrap();
    println!("connect");

}

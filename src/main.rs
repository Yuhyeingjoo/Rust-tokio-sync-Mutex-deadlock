use tokio::sync::Mutex;
use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::cell::RefCell;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
#[tokio::main]
async fn main() {
    let lock = Arc::new(Mutex::new(5));
    let lock2 = Arc::clone(&lock);

    let lock3 = Arc::new(Mutex::new(5));
    let lock4 = Arc::clone(&lock3);

    let h1 = tokio::spawn(async move{
        let a = lock.lock().await;
        println!("h1 {:?}", thread::current().id());
        thread::sleep(Duration::from_millis(2000));
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        let (socket, _) = listener.accept().await.unwrap();
        println!("h1 isetne {:?}", thread::current().id());
        let b = lock4.lock().await;
        println!("h1 get second lock {:?}", thread::current().id());
    });
	let h2 = tokio::spawn(async move{
        let a = lock3.lock().await;
        println!("h2 {:?}", thread::current().id());
        thread::sleep(Duration::from_millis(2000));
        let listener = TcpListener::bind("127.0.0.1:6380").await.unwrap();
        let (socket, _) = listener.accept().await.unwrap();
        println!("h2 isetne {:?}", thread::current().id());
        let b = lock2.lock().await;
        println!("h2 get second lock {:?}", thread::current().id());


    });
	h1.await;
	h2.await;
}

//use std::thread::{sleep, spawn};
use async_std::task::{sleep, spawn};
use std::time::Duration;

async fn sleepus() {
    for i in 1..=10 {
        println!("Sleepus {}", i);
        sleep(Duration::from_millis(500));
    }
}

async fn interruptus() {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000));
    }
}


#[async_std::main]
async fn main() {
    
    //let sleepus = std::thread::spawn(sleepus);
    //let sleepus = std::thread::spawn(||{
    //    for i in 1..=10 {
    //        println!("Sleepus {}", i);
    //        sleep(Duration::from_millis(500));
    //    }
    //});

    sleepus().await;
    interruptus().await;

    //let interruptus = std::thread::spawn(||{
    //    for i in 1..=5 {
    //        println!("Interruptus {}", i);
    //        sleep(Duration::from_millis(1000));
    //    }
    //});

    //sleepus.join().unwrap();
    //interruptus .join().unwrap();
}

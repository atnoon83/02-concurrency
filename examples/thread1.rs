use anyhow::{anyhow, Error, Result};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for idx in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            loop {
                let value = rand::random::<u64>();
                let sleep_time = Duration::from_millis(value % 1000);
                tx.send(format!("thread {} send message: {}", idx, value))?;
                thread::sleep(sleep_time);
                if value % 5 == 0 {
                    break;
                }
            }
            Ok::<(), Error>(println!("thread {} is exit!", idx))
        });
    }

    drop(tx);

    let count = thread::spawn(move || {
        let mut count = 0;
        for received in rx {
            println!("Got: {}", received);
            count += 1;
        }
        count
    })
    .join()
    .map_err(|e| anyhow!("thread join error: {:?}", e))?;

    println!("Received {} messages", count);
    println!("Main thread is done!");
    Ok(())
}

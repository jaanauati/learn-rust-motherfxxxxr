use tokio::{spawn, time::{Duration, sleep}};

async fn one() {
    sleep(Duration::from_millis(100)).await;
    println!("one...");
}

async fn two () {
    sleep(Duration::from_millis(100)).await;
    println!("two...");
}

async fn task (n: i32) {
    sleep(Duration::from_millis(100)).await;
    print!("{} ", n);
}

#[tokio::main]
async fn main() {
    one().await;
    two().await;
    sleep(Duration::from_secs(1)).await;
    let mut handles = vec!();
    let mut i = 0;
    while i <= 1000 {
        handles.push(spawn(task(i)));
        i += 1;
    };
    // now we await for all the concurrent tasks to finish.
    for handle in handles {
        let _ = handle.await;
    }
    println!("");
}

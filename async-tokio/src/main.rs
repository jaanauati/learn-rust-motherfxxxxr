use tokio::{spawn, time::{Duration, sleep}};

async fn one() {
    sleep(Duration::from_millis(100)).await;
    println!("one...");
}

async fn two () {
    sleep(Duration::from_millis(100)).await;
    println!("two...");
}

#[tokio::main]
async fn main() {
    one().await;
    two().await;
    sleep(Duration::from_secs(1)).await;
    let mut i = 0;
    while i <= 20 {
        spawn(one());
        spawn(two());
        i += 1;
    };
    // since spawn just runs the the tasks concurrently we gotta get them some
    // time them to finish. If we omit the following line the program finishes
    // inmediatly without waitting for onw() or two().
    sleep(Duration::from_millis(300)).await;
}

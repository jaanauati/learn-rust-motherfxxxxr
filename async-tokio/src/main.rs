use tokio::{spawn, select, task::{JoinSet}, time::{Duration, sleep}, join};

async fn one() {
    sleep(Duration::from_millis(100)).await;
    println!("one...");
}

async fn two () {
    sleep(Duration::from_millis(100)).await;
    println!("two...");
}

async fn async_fn (n: i32) {
    sleep(Duration::from_millis(100)).await;
    print!("{} ", n);
}

async fn example_one() {
    one().await;
    two().await;
}

// execute async functions/expressions using `tokio::spawn`, which will fire
// them using different "tasks" if possible. This is, async expressions are
// executed in parallel if possible.
async fn example_two() {
    let mut handles = vec!();
    let mut i = 0;
    while i <= 1000 {
        handles.push(spawn(async_fn(i)));
        i += 1;
    };
    // now we await for all the concurrent tasks to finish.
    for handle in handles {
        let _ = handle.await;
    }
    println!("");
}

// the following one is similar to example_two but it uses tokio JoinSet to 
// execute async functions concurrently, this is, using the same "task",
async fn example_three() {
    let tasks: JoinSet<_> = (0..1000).map(|i| async move {
        async_fn(i).await;
    }).collect();
    let _ = tasks.join_all().await;
    // now we await for all the concurrent tasks to finish.
    println!("");
}

async fn example_four() {
    join!(async_fn(0), async_fn(1));
    // now we await for all the concurrent tasks to finish.
    println!("");
}


async fn select_example() {
    select! {
        _ = async_fn(1) => { println!("async_fn 1 completed first") },
        _ = async_fn(2) => { println!("async_fn 2 completed first") },
    }
}

async fn select_example_two() {
    let mut fut1_done = false;
    let mut fut2_done = false;
    // we add a little conditional that guanranties that the loop finishes
    // when both of the functions have been executed.
    while !(fut1_done && fut2_done) {
        select! {
            _ = async_fn(1), if !fut1_done => {
                println!("async_fn 1 completed first");
                fut1_done = true; 
            },
            _ = async_fn(2), if !fut2_done => {
                println!("async_fn 2 completed first");
                fut2_done = true;
            },
        }
    }
}

#[tokio::main]
async fn main() {
    println!("** Example 1: use await to evaluate futures");
    example_one().await;
    sleep(Duration::from_secs(1)).await;
    println!("** Example 2: execute many tasks in parallel using tokio::spawn");
    example_two().await;
    println!("** Example 3: execute many tasks concurrently using tokio::join_all");
    example_three().await;
    println!("** Example 4: execute couple concurrently (and quickly), using tokio::join");
    example_four().await;
    println!("****");
    println!("****");
    println!("****");
    println!("SELECT");
    println!("** Example 5: use select to execute one of two tasks");
    select_example().await;
    println!("** Example 6: use select to wait for completion of the two tasks.");
    select_example_two().await;
}

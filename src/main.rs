mod my_macros;
use tokio::time::{Duration, sleep};

async fn count_up_to(id: char, max: i32) {
    for i in 1..=max {
        println!("{}:{}", id, i);
        sleep(Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    hello!();
    println!("{}", double!(123));
    hoge!(pc, GET);
    pc();
    hoge!(sp, POST);
    sp();

    let tasks = vec![
        tokio::spawn(count_up_to('a', 10)),
        tokio::spawn(count_up_to('b', 10)),
    ];

    // Wait for all tasks to complete.
    for task in tasks {
        task.await.unwrap();
    }
}

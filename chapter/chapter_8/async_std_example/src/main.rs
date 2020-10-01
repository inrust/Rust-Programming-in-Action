use async_std::task;
use std::time::Duration;

fn main() {
    let async_1 = task::spawn(async {
        for i in 1..=5 {
            print_async_1(i).await;
        }
    });

    let async_2 = task::spawn(async {
        for i in 1..=5 {
            print_async_2(i).await;
        }
    });

    for i in 1..=5 {
        println!("number {} from the main!", i);
        task::block_on(async {
            task::sleep(Duration::from_secs(8)).await;
        })
    }

    task::block_on(async_1);
    task::block_on(async_2);
}

async fn print_async_1(i: i32) {
    println!("number {} from the async_1!", i);
    task::sleep(Duration::from_secs(2)).await;
}

async fn print_async_2(i: i32) {
    println!("number {} from the async_2!", i);
    task::sleep(Duration::from_secs(4)).await;
}

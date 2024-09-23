use rand::Rng;

async fn sleep_random(task_id: u32) {
    let mut rng = rand::thread_rng();
    let secs = rng.gen_range(0..5);
    println!("Task {} sleeping for {} secs", task_id, secs);
    tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;
    println!("Task {} returned", task_id);
}

#[tokio::main]
async fn main() {
    for i in 1..=10 {
        tokio::select! {
            _ = sleep_random(1) => println!("Task 1 completed in iteration {}", i),
            _ = sleep_random(2) => println!("Task 2 completed in iteration {}", i),
            _ = sleep_random(3) => println!("Task 3 completed in iteration {}", i),
        }
    }
}

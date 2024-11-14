use tokio_schedule::{every, Job};


fn do_poll() {
    println!("poll");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let poll = every(1).minute()
        .perform(|| async { do_poll(); });

    poll.await;

    Ok(())
}


mod bsky_bot;

use std::env;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender,Receiver};
use tokio_schedule::{every, Job};
use crate::bsky_bot::{BSkyBot, BSkyBotAction};

async fn setup_bsky_bot(rx: Receiver<BSkyBotAction>, user: String, pass: String) -> BSkyBot {
    let mut bsky_bot = BSkyBot::new(rx, "username".to_string(), "password".to_string()).await;
    bsky_bot
}

struct Bots {
    bsky_bot: BSkyBot,
}

fn do_poll(tx: Sender<BSkyBotAction>) {
    println!("poll");
    tx.send(BSkyBotAction::Post {
        title: "Sample".to_string(),
        text: "This is a sample text".to_string(),
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = env::var("BSKY_USER").expect("Environment variable BSKY_USER not set");
    let pass = env::var("BSKY_PASS").expect("Environment variable BSKY_PASS not set");

    let (tx, mut rx) = mpsc::channel(32);

    let mut bsky_bot = setup_bsky_bot(rx, user, pass).await;

    tokio::spawn(async move {
        bsky_bot.start().await;
    });

    {
        let poll = every(1).minute()
            .perform(|| async { do_poll(tx.clone()); });
        poll.await; // This should be outside this block, to prevent tx_copy to leak
    }

    Ok(())
}


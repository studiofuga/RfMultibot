mod bsky_bot;
mod feed;
mod feed_parser;
mod tests;

use crate::bsky_bot::{BSkyBot, BSkyBotAction};
use std::env;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_schedule::{every, Job};

async fn setup_bsky_bot(rx: Receiver<BSkyBotAction>, user: String, pass: String) -> BSkyBot {
    let mut bsky_bot = BSkyBot::new(rx, user, pass).await;
    bsky_bot
}

struct Bots {
    bsky_bot: BSkyBot,
}

async fn do_poll(tx: Sender<BSkyBotAction>) {
    println!("poll");
    let post_res = tx.send(BSkyBotAction::Post {
        title: "Sample".to_string(),
        text: "This is a sample text".to_string(),
    }).await;

    match post_res {
        Ok(_) => { println!("Post create"); },
        Err(err) => { println!("Post create failed: {}", err); }
    }
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

    println!("Started bsky bot");

    let poll = every(1).minute()
        .perform(|| async { do_poll(tx.clone()).await; });
    poll.await;

    Ok(())
}


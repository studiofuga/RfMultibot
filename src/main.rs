mod bsky_bot;
mod feed;
mod feed_parser;
mod tests;
mod storage;
mod set;
mod filter;

use crate::bsky_bot::{BSkyBot, BSkyBotAction};
use std::env;
use log::{debug, error, info, warn};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_schedule::{every, Job};
use crate::feed::Feed;
use crate::feed_parser::parse_feed;

async fn setup_bsky_bot(rx: Receiver<BSkyBotAction>, user: String, pass: String) -> BSkyBot {
    let bsky_bot = BSkyBot::new(rx, user, pass).await;

    bsky_bot
}

async fn do_poll(tx: Sender<BSkyBotAction>) {
    let mut feed = Feed::new("https://ransomfeed.it/rss-complete-Tbot.php".to_string());
    let feedxml = feed.get_feed().await.unwrap();

    match parse_feed(feedxml, &mut feed) {
        Ok(_) => {
            tx.send(BSkyBotAction::NewFeeds {
                feeds: feed.feeds.clone(),
            }).await.unwrap_or_else(|err| error!("Failed sending Feeds: {}", err));
        }
        Err( why ) => {
            // todo: we could save the feed samewhere?
            error!("Cannot parse feed: {}", why);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let user = env::var("BSKY_USER").expect("Environment variable BSKY_USER not set");
    let pass = env::var("BSKY_PASS").expect("Environment variable BSKY_PASS not set");

    info!("Bot starting");

    let (tx, rx) = mpsc::channel(32);

    let mut bsky_bot = setup_bsky_bot(rx, user, pass).await;


    if let Ok(max_posts) = env::var("BSKY_MAX_POST") {
        if let Ok(max_posts_count) = max_posts.parse::<usize>() {
            bsky_bot.max_posts_count = max_posts_count;
        } else {
            warn!("Invalid BSKY_MAX_POST value ({:?}). It should be a valid usize.", env::var("BSKY_MAX_POST"));
        }
    }
    
    tokio::spawn(async move {
        bsky_bot.start().await;
    });

    debug!("Started bsky bot");

    do_poll(tx.clone()).await;
    let poll = every(15).minute()
        .perform(|| async { do_poll(tx.clone()).await; });
    poll.await;

    Ok(())
}


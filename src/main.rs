mod bsky_bot;
mod feed;
mod feed_parser;
mod tests;
mod storage;
mod set;
mod filter;
mod tg_bot;

use crate::bsky_bot::{BSkyBot, BSkyBotAction};
use std::env;
use log::{debug, error, info, warn};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_schedule::{every, Job};
use crate::feed::Feed;
use crate::feed_parser::parse_feed;
use crate::tg_bot::{telegram_bot, Action};

async fn setup_bsky_bot(rx: Receiver<BSkyBotAction>, user: String, pass: String) -> BSkyBot {
    let bsky_bot = BSkyBot::new(rx, user, pass).await;

    bsky_bot
}

#[derive(Clone)]
struct bot_channels {
    tg: Sender<Action>,
    bsky: Sender<BSkyBotAction>,
}

async fn do_poll(channels: bot_channels) {
    let mut feed = Feed::new("https://ransomfeed.it/rss-complete-Tbot.php".to_string());
    let feedxml = match feed.get_feed().await {
        Ok(feed_data) => feed_data,
        Err(err) => {
            error!("Failed to get feed: {}", err);
            return;
        }
    };

    match parse_feed(feedxml, &mut feed) {
        Ok(_) => {
            channels.bsky.send(BSkyBotAction::NewFeeds {
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

    let tgram_key = env::var("TG_BOT_KEY").expect("Environment variable TG_BOT_KEY not set");

    info!("Bot starting");

    let (tx, rx) = mpsc::channel(32);

    let mut bsky_bot = setup_bsky_bot(rx, user, pass).await;

    if let Ok(_) = env::var("BSKY_DISABLE_POST") {
        bsky_bot.post_disabled = true;
        warn!("BSky posting disabled");
    }

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

    let tgbot = telegram_bot::build(&tgram_key);
    let tgtx = tgbot.channel();

    let botchannels = bot_channels {
        tg: tgtx,
        bsky: tx,
    };

    do_poll(botchannels.clone()).await;
    let poll = every(15).minute()
        .perform(|| async { do_poll(botchannels.clone()).await; });
    poll.await;

    Ok(())
}


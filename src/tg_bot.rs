use crate::feed::FeedEntry;
use log::{debug, error, info};
use std::env;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::filter::{DefaultFilter, Filter};
use crate::storage::Storage;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

pub struct telegram_bot {
    bot: Bot,
    channel_id: i64,
    db: Storage,
    tx: Sender<Action>,
    rx: Receiver<Action>,

    pub max_posts_count: usize,
    pub post_disabled: bool,
}

pub enum Action {
    NewFeeds { feeds: Vec<FeedEntry> },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commands: ")]
enum Command {
    #[command(description = "Start command")]
    Start,
}

impl telegram_bot {
    pub async fn build(key: &str, channel_id: i64) -> telegram_bot {
        let (tx, rx) = mpsc::channel(32);
        let bot = Bot::new(key.to_string());

        let handler = Update::filter_message().branch(
            dptree::entry()
                // Filter commands: the next handlers will receive a parsed `SimpleCommand`.
                .filter_command::<Command>()
                // If a command parsing fails, this handler will not be executed.
                .endpoint(command_handler),
        );

        let clonebot = bot.clone();
        tokio::spawn(async move {
            Dispatcher::builder(clonebot, handler)
                //.enable_ctrlc_handler()
                .build()
                .dispatch().await;
        });

        let mut datapath = env::var("DATADIR").unwrap_or("".to_string());
        if !datapath.is_empty() && !datapath.ends_with('/') {
            datapath.push('/');
        }

        let db_filename =
            env::var("TG_DB").unwrap_or_else(|_| format!("{}telegram-bot.db", datapath));
        let db = Storage::new(&db_filename);

        telegram_bot {
            bot,
            channel_id: channel_id,
            db,
            tx,
            rx,
            max_posts_count: 100,
            post_disabled: false,
        }
    }

    pub fn channel(&self) -> Sender<Action> {
        self.tx.clone()
    }

    pub async fn start(&mut self) {
        info!("Telegram bot started");

        while let Some(action) = self.rx.recv().await {
            match action {
                Action::NewFeeds { feeds } => {
                    _ = self.feeds_action(feeds).await;
                }
            }
        }
    }

    async fn feeds_action(&mut self, feeds: Vec<FeedEntry>) {
        let filter = DefaultFilter::new_with_max(self.max_posts_count);
        let to_post = filter.filter(&mut self.db, &feeds);

        for feed in to_post.iter().rev() {
            let published = feed.published.format("%Y-%m-%d %H:%M:%S").to_string();

            let text = format!(
                "ID: {}\n\u{0026A0} {}\n\u{01F977} {}\n\u{01F3AF} {}, {}\n\u{01F517} {}",
                feed.post_id, published, feed.group, feed.title, feed.country, feed.link
            );
            if !self.post_disabled {
                debug!(
                    "New Post for entry: id {} post_id {} group {}",
                    feed.id, feed.post_id, feed.group
                );

                let res = self.bot.send_message(ChatId(self.channel_id), text).await;
                match res {
                    Ok(_) => {
                        debug!("Post sent correctly on telegram");
                    }
                    Err(what) => {
                        error!("Failed to send message on telegram: {}", what);
                    }
                }
            } else {
                debug!("Posting message on telegram: text: {}", text);
            }
        }
    }
}

async fn command_handler(msg: Message, bot: Bot) -> Result<(), teloxide::RequestError> {
    log::info!("Received a message from a group chat.");
    bot.send_message(msg.chat.id, "This is a group chat.")
        .await?;
    respond(())
}

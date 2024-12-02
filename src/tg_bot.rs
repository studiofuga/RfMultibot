use crate::feed::FeedEntry;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};
use crate::filter::DefaultFilter;

pub struct telegram_bot {
    bot: Bot,
    channel_id : i64,
    tx: Sender<Action>,
    rx: Receiver<Action>,
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
    pub async fn build(key: &str) -> telegram_bot {
        let (tx, rx) = mpsc::channel(32);
        let bot = Bot::new(key.to_string());

        let handler = Update::filter_message().branch(
            dptree::entry()
                // Filter commands: the next handlers will receive a parsed `SimpleCommand`.
                .filter_command::<Command>()
                // If a command parsing fails, this handler will not be executed.
                .endpoint(command_handler),
        );

        Dispatcher::builder(bot.clone(), handler)
 //           .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;

        telegram_bot { bot,
            channel_id: -1002073524107,
            tx, rx }
    }

    pub fn channel(&self) -> Sender<Action> {
        self.tx.clone()
    }

    pub async fn start(&mut self) {
        while let Some(action) = self.rx.recv().await {
            match action {
                Action::NewFeeds { feeds } => {
                    _ = self.feeds_action(feeds).await;
                }
            }
        }
    }

    async fn feeds_action(&mut self, feeds: Vec<FeedEntry>) {

        for feed in feeds.iter().rev() {
            let published = feed.published.format("%Y-%m-%d %H:%M:%S").to_string();

            let text = format!(
                "ID: {}\n\u{0026A0} {}\n\u{01F977} {}\n\u{01F3AF} {}, {}\n\u{01F517} {}",
                feed.post_id, published, feed.group, feed.title, feed.country, feed.link
            );
            self.bot.send_message(ChatId(self.channel_id), text).await;
        }
    }
}

async fn command_handler(msg: Message, bot: Bot) -> Result<(), teloxide::RequestError> {
    log::info!("Received a message from a group chat.");
    bot.send_message(msg.chat.id, "This is a group chat.")
        .await?;
    respond(())
}

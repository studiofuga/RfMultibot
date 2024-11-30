use bsky_sdk::api::com::atproto::sync::subscribe_repos::Message;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::feed::FeedEntry;

pub struct telegram_bot {
    bot: Bot,
    tx: Sender<Action>,
    rx: Receiver<Action>
}

pub enum Action {
    NewFeeds{ feeds: Vec<FeedEntry>}
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start
}

impl telegram_bot {
    pub async fn build(key: &str) -> telegram_bot {
        let (tx, rx) = mpsc::channel(32);
        let bot = Bot::from_env();

        telegram_bot{
            bot,
            tx,
            rx
        }
    }

    pub fn channel(&self) -> Sender<Action> {
        self.tx.clone()
    }

    pub async fn start(&self) {
        // NO. Use Dispatcher instead
        Command::repl(&self.bot, bot_reply).await;
    }

}

async fn bot_reply(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    Ok(())
}

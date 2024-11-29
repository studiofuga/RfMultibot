use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::feed::FeedEntry;

pub struct telegram_bot {
    tx: Sender<Action>,
    rx: Receiver<Action>
}

pub enum Action {
    NewFeeds{ feeds: Vec<FeedEntry>}
}

impl telegram_bot {
    pub fn build(key: &str) -> telegram_bot {
        let (tx, rx) = mpsc::channel(32);
        telegram_bot{
            tx,
            rx
        }
    }

    pub fn channel(&self) -> Sender<Action> {
        self.tx.clone()
    }
}
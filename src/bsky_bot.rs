use bsky_sdk::{api, BskyAgent};

use bsky_sdk::api::app::bsky::feed::post as post;
use bsky_sdk::api::types::string;
use chrono::DateTime;
use tokio::sync::mpsc::Receiver;
use crate::bsky_bot;

struct BSkyBotConfig {
    user: String,
    pass: String,
}

pub struct BSkyBot {
    config: BSkyBotConfig,
    agent: BskyAgent,
    rx: Receiver<BSkyBotAction>,
}

pub enum BSkyBotAction {
    Post {
        title: String,
        text: String,
    }
}

impl BSkyBot {
    pub async fn new(rx : Receiver<BSkyBotAction>, user: String, pass: String) -> BSkyBot {
        BSkyBot {
            config: {
                BSkyBotConfig {
                    user,
                    pass,
                }
            },
            agent: bsky_bot::BskyAgent::builder().build().await.unwrap(),
            rx: rx
        }
    }

    pub async fn start(&mut self) {
        self.agent.login(&self.config.user, &self.config.pass);

        while let Some(action) = self.rx.recv().await {
            match action {
                BSkyBotAction::Post { title, text } => {
                    self.agent.create_record(post::RecordData{
                        created_at: string::Datetime::now(),
                        embed: None,
                        entities: None,
                        facets: None,
                        labels: None,
                        langs: None,
                        reply: None,
                        tags: None,
                        text : text,
                    }).await.unwrap();
                }
            }
        }
    }
}

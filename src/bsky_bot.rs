use bsky_sdk::BskyAgent;
use bsky_sdk::api::app::bsky::feed::post as post;
use bsky_sdk::api::types::string;
use tokio::sync::mpsc::Receiver;
use crate::bsky_bot;
use crate::feed::FeedEntry;

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
    },
    NewFeeds {
        feeds: Vec<FeedEntry>
    },
}

impl BSkyBot {
    pub async fn new(rx: Receiver<BSkyBotAction>, user: String, pass: String) -> BSkyBot {
        BSkyBot {
            config: {
                BSkyBotConfig {
                    user,
                    pass,
                }
            },
            agent: bsky_bot::BskyAgent::builder().build().await.unwrap(),
            rx: rx,
        }
    }

    pub async fn start(&mut self) {
        let logged_in = self.agent.login(&self.config.user, &self.config.pass).await;
        match logged_in {
            Ok(_) => {
                println!("Logged in!");
            }
            Err(what) => {
                panic!("Failed to log in: {:?}", what);
            }
        }

        while let Some(action) = self.rx.recv().await {
            match action {
                BSkyBotAction::Post { title, text } => {
                    self.post_action(title, text).await;
                },
                BSkyBotAction::NewFeeds { feeds } => {
                    self.feeds_action(feeds).await;
                }
            }
        }
    }

    async fn post_action(&mut self, title: String, text: String) -> Result<(), ()> {
        println!("New post: {}", &title);
        let res = self.agent.create_record(post::RecordData {
            created_at: string::Datetime::now(),
            embed: None,
            entities: None,
            facets: None,
            labels: None,
            langs: None,
            reply: None,
            tags: None,
            text: text,
        }).await;

        match res {
            Ok(_) => {
                println!("Post sent");
                Ok(())
            }
            Err(what) => {
                panic!("Failed to create post: {:?}", what);
            }
        }
    }

    async fn feeds_action(&mut self, feeds: Vec<FeedEntry>) {
        println!("Feeds: {:?}", feeds.len());
        if feeds.len() > 0 {
            println!("Feed[0]: {:?}", feeds[0].id);
            // Filter
            self.post_action("New Entry".to_string(), feeds[0].title.clone()).await;
        }
    }
}

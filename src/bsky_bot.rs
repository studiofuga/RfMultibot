use std::env;
use bsky_sdk::BskyAgent;
use bsky_sdk::api::app::bsky::feed::post as post;
use bsky_sdk::api::types::string;
use tokio::sync::mpsc::Receiver;
use crate::bsky_bot;
use crate::feed::FeedEntry;
use crate::storage::Storage;

struct BSkyBotConfig {
    user: String,
    pass: String,
}

pub struct BSkyBot {
    config: BSkyBotConfig,
    agent: BskyAgent,
    db: Storage,
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
        let mut datapath = env::var("DATADIR").unwrap_or("".to_string());
        if !datapath.is_empty() && !datapath.ends_with('/') {
            datapath.push('/');
        }

        let db_filename = env::var("BSKY_DB").unwrap_or_else(|_| format!("{}bsky-bot.db", datapath));
        let mut db = Storage::new(&db_filename);

        println!("Using database: {}", db_filename);

        BSkyBot {
            config: {
                BSkyBotConfig {
                    user,
                    pass,
                }
            },
            agent: bsky_bot::BskyAgent::builder().build().await.unwrap(),
            db: db,
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
        let mut to_post: Vec<FeedEntry> = vec![];
        
        for feedEntry in feeds.iter() {
            if feedEntry.published < chrono::Utc::now() - chrono::Duration::days(30) {
                continue;
            }
            if !self.db.has_post(&feedEntry.id) {
                to_post.push(feedEntry.clone());
                self.db.insert(&feedEntry);
            }
        }

        // Sort the feeds to post by their published date, from oldest to newest
        to_post.sort_by(|a, b| a.published.cmp(&b.published));

        let postable : Vec<FeedEntry> = to_post.into_iter().rev().take(10).collect();
        for feed in postable.iter() {
            // Assuming that `feed.published` is a Unix timestamp (in seconds)
            let published = feed.published.format("%Y-%m-%d %H:%M:%S").to_string();

            let text = format!("ID: {}\n\u{0026A0} {}\n\u{01F977} {}\n\u{01F3AF} {}, {}\n\u{01F517} {}",
                feed.post_id, published, feed.group, feed.title, feed.country,
                feed.link
            );

            self.post_action(feed.title.clone(), text).await;
        }
    }
}

use chrono::{DateTime, Utc};
use feed_rs::parser::parse;
use crate::feed::{Feed, FeedEntry};


pub fn parse_feed(xml : String, feed: &mut Feed) -> Result<i32, String> {
    let rss = parse(xml.as_bytes()).unwrap();

    for item in &rss.entries {
        feed.feeds.push(FeedEntry {
            id: item.id.parse().unwrap_or(0),
            title: item.title.as_ref().unwrap().content.clone(),
            link: if item.links.len() > 0 { item.links[0].href.clone() } else { "".to_owned() },
            published: item.published.unwrap_or(Utc::now()),
            country: "none".to_string(),
            group: if item.categories.len() > 0 { item.categories[0].term.clone() } else { "undefined".to_owned() },
        });
    }

    Ok(rss.entries.len() as i32)
}

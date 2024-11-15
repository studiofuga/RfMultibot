use crate::feed::{Feed, FeedEntry};

pub fn parse_feed(xml : String, feed: &mut Feed) -> Result<i32, String> {
    feed.feeds.push(FeedEntry::new("mytitle".to_string()));

    Ok(1)
}

#[cfg(test)]
mod feed_parser_test;

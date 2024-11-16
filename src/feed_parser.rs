use chrono::{DateTime, Utc};
use feed_rs::parser::parse;
use crate::feed::{Feed, FeedEntry};


pub fn parse_feed(xml : String, feed: &mut Feed) -> Result<i32, String> {
    let rss = parse(xml.as_bytes()).unwrap();

    for item in &rss.entries {
        let (post_id, link) = if item.links.len() > 0 {
            let url = url::Url::parse(&item.links[0].href).unwrap();
            let post_id = url.query_pairs()
                .find(|(key, _)| key == "id_post")
                .map(|(_, value)| value.parse::<i32>().unwrap_or(0))
                .unwrap_or(0);
            (post_id, url.as_str().to_string())
        } else {
            (0, "".to_owned())
        };

        feed.feeds.push(FeedEntry {
            id: item.id.to_string(),
            post_id,
            title: item.title.as_ref().unwrap().content.clone(),
            link,
            published: item.published.unwrap_or(Utc::now()),
            country: "none".to_string(),
            group: if item.categories.len() > 0 { item.categories[0].term.clone() } else { "undefined".to_owned() },
        });
    }

    Ok(rss.entries.len() as i32)
}

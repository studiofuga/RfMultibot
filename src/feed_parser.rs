use crate::feed::{Feed, FeedEntry};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::error;
use rss::{Channel};


pub fn parse_feed(xml : String, feed: &mut Feed) -> Result<i32, String> {
    let channel = Channel::read_from(xml.as_bytes()).unwrap();

    for item in &channel.items {

        let (post_id, link) = match &item.link {
            None => {
                (0, "".to_owned())
            }
            Some(link) => {
                let url = url::Url::parse(&link).unwrap();
                let post_id = url.query_pairs()
                    .find(|(key, _)| key == "id_post")
                    .map(|(_, value)| value.parse::<i32>().unwrap_or(0))
                    .unwrap_or(0);
                    (post_id, url.to_string())
                }
        };

        let published = match &item.pub_date {
            None => { Utc::now() }
            Some(tm) => {
                match NaiveDateTime::parse_from_str(tm, "%a, %d %b %Y %H:%M:%S %Z") {
                    Ok(dt) => DateTime::from_naive_utc_and_offset(dt, Utc),
                    Err( x ) => {
                        error!("Cannot parse {}: {}", tm, x);
                        Utc::now() },
                }
            }
        };

        let dc = item.extensions.get_key_value("dc");
        let ext = dc.and_then(|dc| dc.1.get("country"));
        let country = ext.and_then(|ext| { if !ext.is_empty() { ext[0].value.clone() } else {None} }).unwrap_or(String::new()) ;

        let guid = match &item.guid {
            None => "none".to_string(),
            Some(g) => g.value.clone(),
        };

        let title = match &item.title {
            None => { "none".to_string() },
            Some(g) => {g .clone() }
        };
        
        feed.feeds.push(FeedEntry {
            id: guid,
            post_id,
            title: title,
            link,
            published,
            country: country,
            group: if item.categories.len() > 0 { item.categories[0].name.clone() } else { "undefined".to_owned() },
        });
    }

    Ok(channel.items.len() as i32)
}

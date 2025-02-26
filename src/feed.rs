use chrono::{DateTime, Utc};
use reqwest;

#[derive(Debug, PartialEq, Clone)]
pub struct FeedEntry {
    pub id: String,
    pub post_id: i32,
    pub title: String,
    pub link: String,
    pub published: DateTime<Utc>,
    pub country: String,
    pub group: String,
}

pub struct Feed {
    pub url: String,
    pub feeds : Vec<FeedEntry>,
}

impl Feed {
    pub fn new(url: String) -> Self {
        Feed { url, feeds: vec![] }
    }

    pub async fn get_feed(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.url).await?;

        if response.status().is_success() {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to fetch content",
            )))
        }
    }
}

impl FeedEntry {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            post_id: 0,
            title: String::from(id),
            link: "".to_string(),
            published: Utc::now(),
            country: "".to_string(),
            group: "".to_string(),
        }
    }
}
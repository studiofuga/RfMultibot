use crate::feed::FeedEntry;
use crate::set::Set;

pub trait Filter {
    fn filter(&self, set: &mut dyn Set, posts: &Vec<FeedEntry>) -> Vec<FeedEntry>;
}

pub struct DefaultFilter {

}

impl Filter for DefaultFilter {
    fn filter(&self, set: &mut dyn Set, posts: &Vec<FeedEntry>) -> Vec<FeedEntry> {
        let mut to_post: Vec<FeedEntry> = vec![];

        for feed_entry in posts.iter() {
            if feed_entry.published < chrono::Utc::now() - chrono::Duration::days(30) {
                continue;
            }

            if !set.has(&feed_entry.id) {
                to_post.push(feed_entry.clone());
                set.insert(&feed_entry);
            }
        }

        // Sort the feeds to post by their published date, from oldest to newest
        to_post.sort_by(|a, b| b.published.cmp(&a.published));

        to_post
    }
}
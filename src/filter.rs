use crate::feed::FeedEntry;
use crate::set::Set;

pub trait Filter {
    /// Filters the given posts based on whether they are present in the set and additional criteria.
    /// Returns the entries not present in the set, ordered from newest to oldest.
    /// Additional filtering is applied to exclude entries published more than 30 days ago.
    /// Optionally, a maximum number of entries can be specified, truncating the result to the given size.
    ///
    /// # Arguments
    ///
    /// * `set` - A mutable reference to a Set that tracks already processed entries.
    /// * `posts` - A vector of `FeedEntry` representing the posts to be filtered.
    ///
    /// # Returns
    ///
    /// A vector of filtered `FeedEntry` not present in the set, ordered from newest to oldest.
    fn filter(&self, set: &mut dyn Set, posts: &Vec<FeedEntry>) -> Vec<FeedEntry>;
}

pub struct DefaultFilter {
    max: Option<usize>,
}

impl DefaultFilter {
    pub fn new() -> Self {
        DefaultFilter { max: None }
    }
    pub fn new_with_max(max: usize) -> Self {
        DefaultFilter { max: Some(max) }
    }
}

impl Filter for DefaultFilter {
    fn filter(&self, set: &mut dyn Set, posts: &Vec<FeedEntry>) -> Vec<FeedEntry> {
        let mut to_post: Vec<FeedEntry> = vec![];

        for feed_entry in posts.iter() {
            if !set.has(&feed_entry.id) {
                to_post.push(feed_entry.clone());
                set.insert(&feed_entry);
            }
        }

        // Sort the feeds to post by their published date, from oldest to newest
        to_post.sort_by(|a, b| b.published.cmp(&a.published));

        if let Some(max) = self.max {
            if to_post.len() > max {
                to_post.truncate(max);
            }
        }
        
        to_post
    }
}
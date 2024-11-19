use crate::feed::FeedEntry;

pub trait Set {
    fn has(&self, id: &str) -> bool;
    fn insert(&mut self, entry: &FeedEntry);
}
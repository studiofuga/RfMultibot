use crate::feed::FeedEntry;

#[derive(Debug,PartialEq)]
pub enum FeedStorageState {
    Posted,
    Missing,
    Resend,
}

pub trait FeedStorage {
    fn has(&self, id: &str) -> FeedStorageState;

    fn insert(&mut self, entry: &FeedEntry) -> Result<(),()>;

    fn set_resend(&mut self, id: &str) -> Result<(),()>;

    fn set_posted(&mut self, id: &str) -> Result<(),()>;
}
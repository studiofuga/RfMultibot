#[cfg(test)]
use crate::feed::FeedEntry;
use crate::filter::DefaultFilter;
use crate::filter::Filter;
use crate::set::{FeedStorage, FeedStorageState};

use chrono::{TimeZone, Utc};
use std::collections::HashMap;
use crate::set::FeedStorageState::{Missing, Posted, Resend};

struct Bag {
    pub entries: HashMap<String, bool>,
}

impl FeedStorage for Bag {
    fn has(&self, id: &str) -> FeedStorageState {
        let e = self.entries.get(id);
        match e {
            None => Missing,
            Some(resend) => {
                if *resend {
                    FeedStorageState::Resend
                } else {
                    FeedStorageState::Posted
                }
            }
        }
    }

    fn insert(&mut self, entry: &FeedEntry) -> Result<(), ()> {
        self.entries.insert(entry.id.clone(), false);
        Ok(())
    }

    fn set_resend(&mut self, id: &str) -> Result<(), ()> {
        self.set_resend(id, true);
        Ok(())
    }

    fn set_posted(&mut self, id: &str) -> Result<(), ()> {
        self.set_resend(id, false);
        Ok(())
    }
}

impl Bag {
    fn set_resend(&mut self, id: &str, resend: bool) {
        self.entries
            .entry(id.to_string())
            .and_modify(|e| *e = resend);
    }
}

#[test]
pub fn filter_test() {
    // actually: 0 > 2 > 3 > 1
    let entries = vec![
        FeedEntry {
            id: String::from("550e8400-e29b-41d4-a716-446655440000"),
            post_id: 1234,
            title: String::from("Newest"),
            link: String::from("https://example.com/first"),
            published: Utc.timestamp_millis_opt(1731225311).unwrap(),
            country: String::from(""),
            group: String::from("unknown"),
        },
        FeedEntry {
            id: String::from("123e4567-e89b-12d3-a456-42661417400"),
            post_id: 1235,
            title: String::from("Oldest"),
            link: String::from("https://example.com/second"),
            published: Utc.timestamp_millis_opt(1731052511).unwrap(),
            country: String::from(""),
            group: String::from("unknown"),
        },
        FeedEntry {
            id: String::from("f47ac10b-58cc-4372-a567-0e02b2c3d479"),
            post_id: 1236,
            title: String::from("Second Newest"),
            link: String::from("https://example.com/third"),
            published: Utc.timestamp_millis_opt(1731211200).unwrap(),
            country: String::from(""),
            group: String::from("unknown"),
        },
        FeedEntry {
            id: String::from("3d594650-efef-11ea-adc1-0242ac120002"),
            post_id: 1237,
            title: String::from("Thirs newest"),
            link: String::from("https://example.com/fourth"),
            published: Utc.timestamp_millis_opt(1731193200).unwrap(),
            country: String::from(""),
            group: String::from("attacker"),
        },
    ];

    let filter = DefaultFilter::new();
    let mut bag = Bag {
        entries: Default::default(),
    };

    assert_eq!(bag.has("3d594650-efef-11ea-adc1-0242ac120002"), Missing);

    let out = filter.filter(&mut bag, &entries);

    assert_eq!(out.len(), 4);
    assert_eq!(out[0].id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(out[1].id, "f47ac10b-58cc-4372-a567-0e02b2c3d479");
    assert_eq!(out[2].id, "3d594650-efef-11ea-adc1-0242ac120002");
    assert_eq!(out[3].id, "123e4567-e89b-12d3-a456-42661417400");

    let out_empty = filter.filter(&mut bag, &entries);

    assert_eq!(out_empty.len(), 0);

    let mut bag = Bag {
        entries: Default::default(),
    };
    bag.insert(&FeedEntry::new("3d594650-efef-11ea-adc1-0242ac120002"));

    let out_new = filter.filter(&mut bag, &entries);

    assert_eq!(out_new.len(), 3);
    assert_eq!(out_new[0].id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(out_new[1].id, "f47ac10b-58cc-4372-a567-0e02b2c3d479");
    assert_eq!(out_new[2].id, "123e4567-e89b-12d3-a456-42661417400");

    bag.set_resend("3d594650-efef-11ea-adc1-0242ac120002", true);
    assert_eq!(bag.has("3d594650-efef-11ea-adc1-0242ac120002"), Resend);
    let out_new = filter.filter(&mut bag, &entries);
    assert_eq!(out_new.len(), 1);

    bag.set_resend("3d594650-efef-11ea-adc1-0242ac120002", false);
    assert_eq!(bag.has("3d594650-efef-11ea-adc1-0242ac120002"), Posted);
    let out_new = filter.filter(&mut bag, &entries);
    assert_eq!(out_new.len(), 0);

    let lim_filter = DefaultFilter::new_with_max(2);
    let mut bag = Bag {
        entries: Default::default(),
    };

    let out_limited = lim_filter.filter(&mut bag, &entries);

    assert_eq!(out_limited.len(), 2);
    assert_eq!(out_limited[0].id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(out_limited[1].id, "f47ac10b-58cc-4372-a567-0e02b2c3d479");
}

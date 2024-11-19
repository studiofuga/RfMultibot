#[cfg(test)]

use crate::feed::FeedEntry;
use crate::filter::DefaultFilter;
use crate::filter::Filter;
use crate::set::Set;

use chrono::{TimeZone, Utc};
use std::collections::HashSet;

struct Bag {
    pub entries: HashSet<String>
}

impl Set for Bag {
    fn has(&self, id: &str) -> bool {
        self.entries.contains(id)
    }

    fn insert(&mut self, entry: &FeedEntry) {
        self.entries.insert(entry.id.clone());
    }
}

#[test]
pub fn filter_test() {
    let entries = vec![
        FeedEntry {
            id: String::from("550e8400-e29b-41d4-a716-446655440000"),
            post_id: 1234,
            title: String::from("Entry 4"),
            link: String::from("https://example.com/first"),
            published: Utc.with_ymd_and_hms(2024, 11, 10,8, 55, 33).unwrap(),
            country: String::from(""),
            group: String::from("unknown"),
        },
        FeedEntry {
            id: String::from("123e4567-e89b-12d3-a456-42661417400"),
            post_id: 1235,
            title: String::from("Entry 1"),
            link: String::from("https://example.com/second"),
            published: Utc.with_ymd_and_hms(2024, 11, 08,8, 55, 33).unwrap(),
            country: String::from(""),
            group: String::from("unknown"),

        },
        FeedEntry {
            id: String::from("f47ac10b-58cc-4372-a567-0e02b2c3d479"),
            post_id: 1236,
            title: String::from("Entry 3"),
            link: String::from("https://example.com/third"),
            published: Utc.with_ymd_and_hms(2024, 11, 10,5, 55, 33).unwrap(),
            country: String::from(""),
            group: String::from("unknown"),
        },
        FeedEntry {
            id: String::from("3d594650-efef-11ea-adc1-0242ac120002"),
            post_id: 1237,
            title: String::from("Entry 2"),
            link: String::from("https://example.com/fourth"),
            published: Utc.with_ymd_and_hms(2024, 11, 10,0, 55, 33).unwrap(),
            country: String::from(""),
            group: String::from("attacker"),
        },
    ];

    let filter = DefaultFilter::new();
    let mut bag = Bag{ entries: Default::default() };

    let out = filter.filter(&mut bag, &entries);
    
    assert_eq!(out.len(), 4);
    assert_eq!(out[0].id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(out[1].id, "f47ac10b-58cc-4372-a567-0e02b2c3d479");
    assert_eq!(out[2].id, "3d594650-efef-11ea-adc1-0242ac120002");
    assert_eq!(out[3].id, "123e4567-e89b-12d3-a456-42661417400");

    let out_empty = filter.filter(&mut bag, &entries);
    
    assert_eq!(out_empty.len(), 0);
    
    let mut bag = Bag { entries: Default::default() };
    bag.insert(&FeedEntry::new("3d594650-efef-11ea-adc1-0242ac120002"));

    let out_new = filter.filter(&mut bag, &entries);

    assert_eq!(out_new.len(), 3);
    assert_eq!(out_new[0].id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(out_new[1].id, "f47ac10b-58cc-4372-a567-0e02b2c3d479");
    assert_eq!(out_new[2].id, "123e4567-e89b-12d3-a456-42661417400");

    let lim_filter = DefaultFilter::new_with_max(2);
    let mut bag = Bag { entries: Default::default() };

    let out_limited = lim_filter.filter(&mut bag, &entries);

    assert_eq!(out_limited.len(), 2);
    assert_eq!(out_limited[0].id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(out_limited[1].id, "f47ac10b-58cc-4372-a567-0e02b2c3d479");
}
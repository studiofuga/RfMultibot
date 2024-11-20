#[cfg(test)]
use chrono::DateTime;

use crate::feed::Feed;
use crate::feed::FeedEntry;
use crate::feed_parser;

#[test]
pub fn feed_parser_test() {
    let xml = r#"<?xml version='1.0' encoding='UTF-8' ?>
<rss version='2.0'>
    <channel>
        <title>Ransom Feed | RSS Complete</title>
        <link>http://www.ransomfeed.it/</link>
        <description>Ransomware victims RSS
            <img referrerpolicy="no-referrer-when-downgrade"
                 src="https://matomo.ransomfeed.it/matomo.php?idsite=1&amp;rec=1" style="border:0" alt=""/>
        </description>
        <language>en-us</language>
        <item xmlns:dc='ns:1'>
            <title>www.gob.mx</title>
            <link>http://www.ransomfeed.it/index.php?page=post_details&amp;id_post=18643</link>
            <guid>fe3b673e6693cf4a21e6e1b9a26e72c3</guid>
            <post_id>18643</post_id>
            <pubDate>Fri, 15 Nov 2024 14:50:18 UTC</pubDate>
            <dc:creator>RansomFeed</dc:creator>
            <dc:country>Mexico</dc:country>
            <category>ransomhub</category>
        </item>
        <item xmlns:dc='ns:1'>
            <title>A AND O IT Group</title>
            <link>http://www.ransomfeed.it/index.php?page=post_details&amp;id_post=18642</link>
            <guid>3467a9a96db4f23d5f45b0f5eaeaff06</guid>
            <post_id>18642</post_id>
            <pubDate>Fri, 15 Nov 2024 13:04:06 UTC</pubDate>
            <dc:creator>RansomFeed</dc:creator>
            <dc:country>UK</dc:country>
            <category>hunters</category>
        </item>
    </channel>
</rss>
"#;

    let mut feed = Feed::new("something".to_string());
    let parsed = feed_parser::parse_feed(xml.to_string(), &mut feed);

    assert_eq!(parsed.unwrap(), 2);
    assert_eq!(feed.feeds.len(), 2);

    assert_eq!(feed.feeds[0],
        FeedEntry{
            id: "fe3b673e6693cf4a21e6e1b9a26e72c3".to_string(),
            post_id: 18643,
            title: "www.gob.mx".to_string(),
            link: "http://www.ransomfeed.it/index.php?page=post_details&id_post=18643".to_string(),
            published: DateTime::from(DateTime::parse_from_rfc3339("2024-11-15T14:50:18Z").unwrap()),
            country: "Mexico".to_string(),
            group: "ransomhub".to_string(),
        }
    );

    assert_eq!(feed.feeds[1],
        FeedEntry{
            id: "3467a9a96db4f23d5f45b0f5eaeaff06".to_string(),
            post_id: 18642,
            title: "A AND O IT Group".to_string(),
            link: "http://www.ransomfeed.it/index.php?page=post_details&id_post=18642".to_string(),
            published: DateTime::from(DateTime::parse_from_rfc3339("2024-11-15T13:04:06Z").unwrap()),
            country: "UK".to_string(),
            group: "hunters".to_string(),
        }
    );
}

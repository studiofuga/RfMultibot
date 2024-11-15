#[cfg(test)]
use chrono::format::parse;
use crate::feed::Feed;
use crate::feed_parser;

#[test]
pub fn feed_parser_test() {
    let xml = r#"<?xml version='1.0' encoding='UTF-8' ?>
<rss version='2.0'>
    <channel>
        <title>Ransom Feed | RSS Complete</title>
        <link>http://www.ransomfeed.it/</link>
        <description>Ransomware victims RSS
            <img referrerpolicy=\"no-referrer-when-downgrade\"
            src=\"https://matomo.ransomfeed.it/matomo.php?idsite=1&amp;rec=1\" style="border:0" alt=\"\"/>
        </description>
        <language>en-us</language>
        <item xmlns:dc='ns:1'>
            <title>fortinainvestments.com</title>
            <link>http://www.ransomfeed.it/index.php?page=post_details&amp;id_post=18641</link>
            <guid>f31b56f9432b4f03b3b5f4611be3fd96</guid>
            <post_id>18641</post_id>
            <pubDate>Fri, 15 Nov 2024 06:57:37 UTC</pubDate>
            <dc:creator>RansomFeed</dc:creator>
            <country>Malta</country>
            <category>ransomhub</category>
        </item>
        <item xmlns:dc='ns:1'>
            <title>BluMed Health</title>
            <link>http://www.ransomfeed.it/index.php?page=post_details&amp;id_post=18640</link>
            <guid>44119006254708ef096f25a96700dfb1</guid>
            <post_id>18640</post_id>
            <pubDate>Fri, 15 Nov 2024 00:40:22 UTC</pubDate>
            <dc:creator>RansomFeed</dc:creator>
            <country>India</country>
            <category>killsec</category>
        </item>
    </channel>
</rss>
"#;

    let mut feed = Feed::new("something".to_string());
    let parsed = feed_parser::parse_feed(xml.to_string(), &mut feed);

    assert_eq!(parsed.unwrap(), 2);
    assert_eq!(feed.feeds.len(), 2);
}

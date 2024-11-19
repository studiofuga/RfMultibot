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
            <id>18643</id>
            <pubDate>Fri, 15 Nov 2024 14:50:18 UTC</pubDate>
            <dc:creator>RansomFeed</dc:creator>
            <description>
                <![CDATA[Ransomware group called <b>ransomhub</b> claims attack for <b>www.gob.mx</b>. The target comes from <b>Mexico</b>. <img referrerpolicy="no-referrer-when-downgrade" src="https://www.ransomfeed.it/matomo/matomo.php?idsite=1&amp;rec=1&amp;action_name=RSS-slow" style="border:0" alt="" /><br />We identify this attack with following <b>hash code</b>: <i>bd759f93965e708bf4fea0d8d93fe07a4a156272c32f5401fc82c543e64e4bac</i> (ID: 18643)<br /><br />Target victim <b>website</b>: <i>N/D</i>]]></description>
            <category>ransomhub</category>
        </item>
        <item xmlns:dc='ns:1'>
            <title>A&amp;O IT Group</title>
            <link>http://www.ransomfeed.it/index.php?page=post_details&amp;id_post=18642</link>
            <guid>3467a9a96db4f23d5f45b0f5eaeaff06</guid>
            <id>18642</id>
            <pubDate>Fri, 15 Nov 2024 13:04:06 UTC</pubDate>
            <dc:creator>RansomFeed</dc:creator>
            <description>
                <![CDATA[Ransomware group called <b>hunters</b> claims attack for <b>A&amp;O IT Group</b>. The target comes from <b>UK</b>. <img referrerpolicy="no-referrer-when-downgrade" src="https://www.ransomfeed.it/matomo/matomo.php?idsite=1&amp;rec=1&amp;action_name=RSS-slow" style="border:0" alt="" /><br />We identify this attack with following <b>hash code</b>: <i>85064c33f5a01b1cd98c03f9a6442b7d559b3314f39db060d179f6dd31944ebc</i> (ID: 18642)<br /><br />Target victim <b>website</b>: <i>N/D</i>]]></description>
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
            country: "".to_string(),
            group: "ransomhub".to_string(),
        }
    );
}

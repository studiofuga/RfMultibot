use log::error;
use crate::feed::FeedEntry;
use crate::set::Set;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

pub struct Storage {
    handle: Connection,
}

impl Storage {
    pub fn new(file: &str) -> Storage {
        let mut storage = Storage {
            handle: rusqlite::Connection::open(file).unwrap(),
        };

        let migrations = Migrations::new(vec![
            M::up(r##"
            create table posts (
                    id        TEXT                          not null,
                    pid       integer,
                    title     TEXT,
                    link      TEXT,
                    published integer default 'unixepoch()' not null,
                    country   TEXT,
                    attacker  TEXT    default unknown       not null
                );
            create unique index id_index on posts (id);
            "##),

            // In the future, add more migrations here:
            //M::up("ALTER TABLE friend ADD COLUMN email TEXT;"),
        ]);

        // Apply some PRAGMA, often better to do it outside of migrations

        migrations.to_latest(&mut storage.handle).unwrap();

        storage
    }

    pub fn in_memory() -> Storage {
        Storage::new(":memory:")
    }

    pub fn has_post(&self, guid: &str) -> bool {
        self.handle.query_row("SELECT 1 FROM posts WHERE id = ?", params![guid], |_| Ok(true)).is_ok()
    }

    pub fn insert(&mut self, feed: &FeedEntry) {
        _= self.handle.execute("INSERT INTO posts (id, pid, title, link, published, country, attacker) VALUES(?,?,?,?,?,?,?)",
                            params![feed.id, feed.post_id, feed.title, feed.link, feed.published.timestamp(), feed.country, feed.group])
            .map_err(|err|{ error!("Error while inserting feed: {:?}: {}", feed.id, err.to_string()); } );
    }
}

impl Set for Storage {
    fn has(&self, id: &str) -> bool {
        self.has_post(id)
    }

    fn insert(&mut self, entry: &FeedEntry) {
        self.insert(entry);
    }
}

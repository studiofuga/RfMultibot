use log::error;
use crate::feed::FeedEntry;
use crate::set::{FeedStorage, FeedStorageState};
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations};
use include_dir::{include_dir, Dir};
use teloxide::dispatching::dialogue::Storage;
use crate::set::FeedStorageState::{Missing, Posted, Resend};

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub struct SqliteStorage {
    handle: Connection,
}

impl SqliteStorage {
    pub fn new(file: &str) -> SqliteStorage {
        let mut storage = SqliteStorage {
            handle: rusqlite::Connection::open(file).unwrap(),
        };

        let migrations = Migrations::from_directory(&MIGRATIONS_DIR).unwrap();

        // Apply some PRAGMA, often better to do it outside of migrations

        migrations.to_latest(&mut storage.handle).unwrap();

        storage
    }

    pub fn in_memory() -> SqliteStorage {
        SqliteStorage::new(":memory:")
    }

    pub fn has_post(&self, guid: &str) -> FeedStorageState {
        let resend = self.handle.query_row::<i64,_,_>("SELECT resend FROM posts WHERE id = ?", params![guid], |r| r.get(0));

        match resend {
            Ok(r) => { if r == 0 { Posted } else { Resend } }
            Err(_) => { Missing }
        }
    }

    pub fn insert(&mut self, feed: &FeedEntry) {
        _= self.handle.execute("INSERT INTO posts (id, pid, title, link, published, country, attacker) VALUES(?,?,?,?,?,?,?)",
                            params![feed.id, feed.post_id, feed.title, feed.link, feed.published.timestamp(), feed.country, feed.group])
            .map_err(|err|{ error!("Error while inserting feed: {:?}: {}", feed.id, err.to_string()); } );
    }

    fn set_resend_flag(&mut self, guid: &str, flag: bool) {
        _ = self.handle.execute("UPDATE posts SET resend = ? WHERE id = ?", params![{ if flag { 1 } else { 0 }},guid])
            .map_err(|err|{ error!("Error while updating posts : {:?}", err ) } );
    }

    pub fn set_to_resend(&mut self, giud: &str) {
        self.set_resend_flag(giud, true);
    }

    pub fn set_published(&mut self, guid: &str) {
        self.set_resend_flag(guid, false);
    }

    pub fn remove(&mut self, guid: &str) {
        _ = self.handle.execute("DELETE FROM posts WHERE id = ?", params![guid]);
    }
}

impl FeedStorage for SqliteStorage {
    fn has(&self, id: &str) -> FeedStorageState {
        self.has_post(id)
    }

    fn insert(&mut self, entry: &FeedEntry) -> Result<(),()> {
        self.insert(entry);
        Ok(())
    }

    fn set_resend(&mut self, id: &str) -> Result<(), ()> {
        self.set_to_resend(id);
        Ok(())
    }

    fn set_posted(&mut self, id: &str) -> Result<(), ()> {
        self.set_published(id);
        Ok(())
    }
}

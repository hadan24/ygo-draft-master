#![allow(unused)]

use std::str::FromStr;

use sqlx::sqlite::{
    SqliteConnectOptions,
    SqlitePool,
    SqlitePoolOptions,
    SqliteJournalMode
};

const DB_URL: &str = "sqlite:cards.db";
pub struct Database(SqlitePool);

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // check docs for defaults, un- or re-comment as needed
        // https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqliteConnectOptions.html
        let opts = SqliteConnectOptions::from_str(DB_URL)
            .expect("Check the DB url")
            //.foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            // app & thus tables not expected to get very large, but recheck this as needed
            .optimize_on_close(true, 400)   // set to "recommended"
            ;
        let pool = SqlitePoolOptions::new()
            .max_connections(50)
            .max_lifetime(Some(std::time::Duration::from_secs(2)))
            .connect_with(opts)
            .await?;
        
        Ok(Database(pool))
    }
}
pub fn open_db() {
    // blobs are JSON
    let create_cards_cmd = "
        CREATE TABLE cards IF NOT EXISTS (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            desc    TEXT NOT NULL,
            img     BLOB NOT NULL,
            ctype   TEXT NOT NULL CHECK(ctype IN ('Monster', 'Spell', 'Trap')),
            atk     INTEGER,
            def     INTEGER,
            level   INTEGER,
            attribute   TEXT CHECK(attribute IN ('Fire', 'Water', 'Earth', 'Wind', 'Dark', 'Light', 'Divine')),
            mtype   TEXT CHECK(mtype IN ('Aqua', 'Beast', 'BeastWarrior', 'CreatorGod', 'Cyberse', 'Dinosaur', 'DivineBeast', 'Dragon', 'Fairy', 'Fiend', 'Fish', 'Illusion', 'Insect', 'Machine', 'Plant', 'Psychic', 'Pyro', 'Reptile', 'Rock', 'SeaSerpent', 'Spellcaster', 'Thunder', 'Warrior', 'WingedBeast', 'Wyrm', 'Zombie')),
            flavor  TEXT CHECK(mtype IN ('Normal', 'Effect', 'Ritual', 'Fusion', 'Synchro', 'Xyz', 'Link')),
            linkmarkers BLOB,
            subtypes    BLOB,
            pend_eff    TEXT,
            scale   INTEGER
        )
    ";
}

pub fn insert_card() {
    todo!()
}


use serde::Deserialize;
use std::str::FromStr;
use sqlx::{
    Row, 
    sqlite::{
        SqliteConnectOptions,
        SqliteJournalMode,
        SqlitePool,
        SqlitePoolOptions
    },
    Transaction
};
use thiserror::Error;
use crate::card::{
    response_card,
    ygo_card
};


const DB_URL: &str = "sqlite:cards.db";
pub struct Database(SqlitePool);

#[derive(Deserialize)]
struct YGOProVersionResponse {
    #[serde(alias="0")]
    inner: VersionInner
}
#[derive(Deserialize)]
struct VersionInner {
    #[serde(alias="database_version")]
    value: std::rc::Rc<str>
}
impl YGOProVersionResponse {
    fn value(&self) -> f64 {
        self.inner.value.parse().expect("Check if YGOPro API for version checks has changed")
    }
}

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

    pub async fn update_cards_db(&mut self) -> Result<(), DatabaseError> {
        let pool = &self.0;
        let client = reqwest::Client::new();

        sqlx::query("CREATE TABLE IF NOT EXISTS cards_version (
            id  INTEGER PRIMARY KEY NOT NULL,
            version_num REAL NOT NULL
        )").execute(pool)
        .await?;

        let (ygopro_version, maybe_db_version) = tokio::join!(
            client.get("https://db.ygoprodeck.com/api/v7/checkDBVer.php").send(),
            sqlx::query("SELECT version_num FROM cards_version WHERE id = 1").fetch_optional(pool)
        );
        let ygopro_version = ygopro_version?
            .json::<YGOProVersionResponse>()
            .await?
            .value();

        let mut txn = pool.begin().await?;
        match maybe_db_version? {
            Some(row) if row.get::<f64, &str>("version_num") == ygopro_version
                => return Ok(()),
            Some(_) => {sqlx::query("UPDATE cards_version SET version_num = $1")
                .bind(ygopro_version)
                .execute(&mut *txn)
                .await?;
            },
            None => {sqlx::query("INSERT INTO cards_version (id, version_num) VALUES ($1, $2)")
                .bind(1)
                .bind(ygopro_version)
                .execute(&mut *txn)
                .await?;
            }
        }

        // if same version, would've returned already, go on to update cards
        let (cards_tbl_result, resp) = tokio::join!(
            sqlx::query("CREATE TABLE IF NOT EXISTS cards (
                id      INTEGER PRIMARY KEY,
                name    TEXT NOT NULL,
                desc    TEXT NOT NULL,
                img     BLOB NOT NULL,
                ctype   TEXT NOT NULL CHECK(ctype IN ('Monster', 'Spell', 'Trap')),
                atk     INTEGER,
                def     INTEGER,
                level   INTEGER,
                attr    TEXT CHECK(attr IN ('Fire', 'Water', 'Earth', 'Wind', 'Dark', 'Light', 'Divine')),
                mtype   TEXT CHECK(mtype IN ('Aqua', 'Beast', 'BeastWarrior', 'CreatorGod', 'Cyberse', 'Dinosaur', 'DivineBeast', 'Dragon', 'Fairy', 'Fiend', 'Fish', 'Illusion', 'Insect', 'Machine', 'Plant', 'Psychic', 'Pyro', 'Reptile', 'Rock', 'SeaSerpent', 'Spellcaster', 'Thunder', 'Warrior', 'WingedBeast', 'Wyrm', 'Zombie')),
                stype   TEXT CHECK (stype IN ('Normal', 'Equip', 'Field', 'Continuous', 'QuickPlay', 'Ritual')),
                ttype   TEXT CHECK (ttype IN ('Normal', 'Continuous', 'Counter')),
                flavor  TEXT CHECK(flavor IN ('Normal', 'Effect', 'Ritual', 'Fusion', 'Synchro', 'Xyz', 'Link')),
                linkmarkers BLOB,
                subtypes    BLOB,
                pend_eff    TEXT,
                scale   INTEGER
            )").execute(&mut *txn),

            client.get("https://db.ygoprodeck.com/api/v7/cardinfo.php?").query(&[("format", "tcg")]).send()
        );

        cards_tbl_result?;
        let resp = resp?.json::<response_card::YGOProResponse>()
            .await?;
        let ygopro_cards = tokio::task::spawn_blocking(move || {
            resp.data
                .into_iter()
                .filter_map(|rcard| {
                    if rcard.is_invalid() { return None; }

                    let id = rcard.id;
                    match ygo_card::YGOCard::new_from_response(rcard) {
                        Ok(card) => Some(Ok(card)),
                        Err(e) => Some(Err(DatabaseError::CardCreationError { invalid_card_id: id, error: e }))
                    }
                })
                .collect::< Result<Vec<ygo_card::YGOCard>, DatabaseError> >()
        }).await
        .expect("Processing ResponseCards should not panic, and this task should only get cancelled if server is cancelled")?;

        for c in ygopro_cards {
            self.insert_or_update_card(&c, Some(&mut txn)).await?;
        }
        txn.commit().await?;
        Ok(())
    }

    async fn insert_or_update_card<'c>(&mut self, card: &ygo_card::YGOCard, current_txn: Option<&mut Transaction<'c, Sqlite>>)
        -> Result<(), sqlx::Error>
    {
        
        let query = sqlx::query("INSERT INTO cards 
            (id, name, desc, img, ctype, atk, def, level, attr, mtype, stype, ttype, flavor, linkmarkers, subtypes, pend_eff, scale)
            VALUES 
            ($1, $2,   $3,   $4,  $5,    $6,  $7,  $8,    $9,   $10,   $11,   $12,   $13,    $14,         $15,      $16,      $17)
            ON CONFLICT (id) DO UPDATE SET 
            name = excluded.name,   desc = excluded.desc,   img = excluded.img,     ctype = excluded.ctype,
            atk = excluded.atk,     def = excluded.def,     level = excluded.level, attr = excluded.attr,
            mtype = excluded.mtype, stype = excluded.stype, ttype = excluded.ttype,
            flavor = excluded.flavor,       linkmarkers = excluded.linkmarkers,
            subtypes = excluded.subtypes,   pend_eff = excluded.pend_eff,       scale = excluded.scale
        ")
            .bind(card.id)
            .bind(&card.name)
            .bind(&card.desc)
            .bind(serde_json::to_string(&card.img).expect("Check img link serialization").as_bytes())
            .bind(Self::get_ctype_binding(&card.ctype));

        use ygo_card::CardType;
        use std::sync::Arc;
        let query = match &card.ctype {
            CardType::Spell(stype) => query.bind(Option::<i16>::None)   // atk
                .bind(Option::<i16>::None)  // def
                .bind(Option::<u8>::None)   // level
                .bind(Option::<Arc<str>>::None)   // attribute
                .bind(Option::<Arc<str>>::None)   // mtype
                .bind(stype.to_string())
                .bind(Option::<Arc<str>>::None)   // ttype
                .bind(Option::<Arc<str>>::None)   // monster flavor
                .bind(Option::<Vec<u8>>::None)  // linkmarkers blob
                .bind(Option::<Vec<u8>>::None)  // subtype blob
                .bind(Option::<Arc<str>>::None).bind(Option::<u8>::None), // pend eff + scale
            CardType::Trap(ttype) => query.bind(Option::<i16>::None)   // atk
                .bind(Option::<i16>::None)  // def
                .bind(Option::<u8>::None)   // level
                .bind(Option::<Arc<str>>::None)   // attribute
                .bind(Option::<Arc<str>>::None)   // mtype
                .bind(Option::<Arc<str>>::None)   // stype
                .bind(ttype.to_string())
                .bind(Option::<Arc<str>>::None)   // monster flavor
                .bind(Option::<Vec<u8>>::None)  // linkmarker blob
                .bind(Option::<Vec<u8>>::None)  // subtype blob
                .bind(Option::<Arc<str>>::None).bind(Option::<u8>::None), // pend eff + scale
            CardType::Monster { atk, def, level, attribute, mtype, flavor, subtypes, pendulum } => {
                // i wonder if these to_owned()'s will come back to bite me
                let linkmarker_blob = match flavor {
                    ygo_card::MonsterFlavor::Link(markers) => Some(serde_json::to_string(markers)
                        .expect("Check link marker serialization")
                        .into_bytes()),
                    _ => None
                };
                let (pend_eff_bind, scale_bind) = match pendulum {
                    Some(p) => (Some(p.pend_eff.clone()), Some(p.scale)),
                    None => (Option::<std::sync::Arc<str>>::None, Option::<u8>::None)
                };
                query.bind(atk)
                    .bind(def)
                    .bind(level)
                    .bind(attribute.to_string())
                    .bind(mtype.to_string())
                    .bind(Option::<Arc<str>>::None)   // stype
                    .bind(Option::<Arc<str>>::None)   // ttype
                    .bind(flavor.to_string())
                    .bind(linkmarker_blob)
                    .bind(serde_json::to_string(&subtypes).expect("Check subtypes serialization").as_bytes())
                    .bind(pend_eff_bind)
                    .bind(scale_bind)
            }
        };

        if let Some(txn) = current_txn {
            // why 2 derefs???? https://stackoverflow.com/questions/78322517/pass-a-transaction-to-a-function-and-do-a-query
            query.execute(&mut **txn).await?;
        } else {
            query.execute(&self.0).await?;
        }

        Ok(())
    }

    fn get_ctype_binding(ctype: &ygo_card::CardType) -> std::rc::Rc<str> {
        use ygo_card::CardType;
        match ctype {
            CardType::Monster{..} => std::rc::Rc::from("Monster"),
            CardType::Spell(_) => std::rc::Rc::from("Spell"),
            CardType::Trap(_) => std::rc::Rc::from("Trap")
        }
    }
}


#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("couldn't get data from YGOProDeck: {0}")]
    YGOProAccessFailed(#[from] reqwest::Error),
    #[error("{0}")]
    DbError(#[from] sqlx::Error),
    #[error("couldn't create card {invalid_card_id} from response: {error}")]
    CardCreationError {
        invalid_card_id: u32,
        error: ygo_card::CardCreationError
    }
}
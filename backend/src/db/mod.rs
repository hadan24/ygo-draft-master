use sqlx::*;

// enum CardType {
//     Monster {
//         atk: i16,
//         def: i16,
//         level: u8,
//         attribute: Attribute,
//         mtype: MonsterType,
//         flavor: MonsterFlavor,
//         subtypes: Rc<[SubType]>,
//         pendulum: Option<Pendulum>,
//     },
//     Spell(SpellType),
//     Trap(TrapType)
// }

pub fn open_db() -> Result<Connection> {
    let cxn = Connection::open("./cards.db")?;

    let create_cards_cmd = "
        CREATE TABLE cards IF NOT EXISTS (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            desc    TEXT NOT NULL,
            img     BLOB NOT NULL,
            ctype   TEXT NOT NULL,
            atk     INTEGER,
            def     INTEGER,
            level   INTEGER,
            attribute   TEXT,
            mtype   TEXT,
            linkmarkers BLOB,
            flavor  TEXT,
            subtypes    BLOB,
            pend_eff    TEXT,
            scale   INTEGER
        )
    ";
    cxn.execute(create_cards_cmd, ())?;

    Ok(cxn)
}

pub fn insert_card(cxn: Connection, card: &crate::card::ygo_card::YGOCard) -> Result<_> {
    todo!()
}
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::card::{
    Attribute,
    LinkMarkers,
    ImgLinks
};


#[derive(Deserialize, Serialize)]
struct YGOCard {
    id: u32,
    name: Rc<str>,
    desc: Rc<str>,
    ctype: CardType,
    img: ImgLinks
}

#[derive(Deserialize, Serialize)]
enum CardType {
    Monster {
        atk: i16,
        def: i16,
        level: u8,
        flavor: MonsterFlavor,
        attribute: Attribute,
        mtype: MonsterType,
        subtypes: Rc<[SubType]>,
        pendulum: Option<Pendulum>,
    },
    Spell(SpellType),
    Trap(TrapType)
}

#[derive(Deserialize, Serialize)]
enum MonsterFlavor {
    Normal,
    Effect,
    Ritual,
    Fusion,
    Synchro,
    Xyz,
    Link(Rc<[LinkMarkers]>)
}
#[derive(Deserialize, Serialize)]
enum MonsterType {
    Aqua,
    Beast,
    BeastWarrior,
    CreatorGod,
    Cyberse,
    Dinosaur,
    DivineBeast,
    Dragon,
    Fairy,
    Fiend,
    Fish,
    Insect,
    Machine,
    Plant,
    Psychic,
    Pyro,
    Reptile,
    Rock,
    SeaSerpent,
    Spellcaster,
    Thunder,
    Warrior,
    WingedBeast,
    Wyrm,
    Zombie,
}
#[derive(Deserialize, Serialize)]
enum SubType {
    Flip,
    Tuner,
    Gemini,
    Spirit,
    Toon,
    Union
}
#[derive(Deserialize, Serialize)]
struct Pendulum {
    pend_eff: Rc<str>,
    scale: u8
}

#[derive(Deserialize, Serialize)]
enum SpellType {
    Normal,
    Equip,
    Field,
    Continuous,
    QuickPlay,
    Ritual
}
#[derive(Deserialize, Serialize)]
enum TrapType {
    Normal,
    Continuous,
    Counter
}

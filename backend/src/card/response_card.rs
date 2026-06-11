use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::card::{
    Attribute,
    LinkMarkers,
    ImgLinks
};

#[derive(Debug, Deserialize, Serialize)]
pub struct YGOProResponse{
    data: Vec<ResponseCard>
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ResponseCard {
    pub(super) id: u32,
    pub(super) name: Rc<str>,
    #[serde(alias="type")]
    pub(super) card_type: Rc<str>,
    pub(super) desc: Rc<str>,
    pub(super) race: Race,
    pub(super) card_images: Rc<[ImgLinks]>,

    #[serde(flatten)]
    pub(super) monster_data: Option<MonsterData>
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct MonsterData {
    pub(super) atk: u16,
    pub(super) def: Option<u16>,
    pub(super) level: u8,
    pub(super) attribute: Attribute,
    pub(super) pend_desc: Option<Rc<str>>,
    pub(super) monster_desc: Option<Rc<str>>,
    pub(super) scale: Option<u8>,
    pub(super) linkmarkers: Option<Vec<LinkMarkers>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) enum Race {
    Aqua,
    Beast,
    #[serde(alias="Beast-Warrior")]
    BeastWarrior,
    #[serde(alias="Creator God")]
    CreatorGod,
    Cyberse,
    Dinosaur,
    #[serde(alias="Divine-Beast")]
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
    #[serde(alias="Sea Serpent")]
    SeaSerpent,
    Spellcaster,
    Thunder,
    Warrior,
    #[serde(alias="Winged Beast")]
    WingedBeast,
    Wyrm,
    Zombie,

    Normal,
    Equip,
    Field,
    Continuous,
    #[serde(alias="Quick-Play")]
    QuickPlay,
    Ritual,
    Counter
}

use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::card::{
    Attribute,
    LinkMarkers,
    ImgLinks,
    response_card
};


#[derive(Deserialize, Serialize)]
struct YGOCard {
    id: u32,
    name: Rc<str>,
    desc: Rc<str>,
    ctype: CardType,
    img: ImgLinks
}
impl YGOCard {
    pub fn new_from_response(r: response_card::ResponseCard) -> Result<Self, CardCreationError> {
        if r.card_type.contains("Skill") { 
            Err(CardCreationError::InvalidType { given: InvalidType::Skill })
        }
        else if r.card_type.contains("Token") {
            Err(CardCreationError::InvalidType { given: InvalidType::Token })
        }
        else if r.card_type.contains("Spell") {
            Ok(YGOCard {
                id: r.id,   name: r.name,   desc: r.desc,
                ctype: CardType::Spell(r.race.try_into()?),
                img: r.card_images[0].clone()
            })
        }
        else if r.card_type.contains("Trap") {
            Ok(YGOCard {
                id: r.id,   name: r.name,   desc: r.desc,
                ctype: CardType::Trap(r.race.try_into()?),
                img: r.card_images[0].clone()
            })
        }
        else {  // should be a Monster, check api reference if not
            Self::monster_from_response(r)
        }
    }

    fn monster_from_response(r: response_card::ResponseCard) -> Result<Self, CardCreationError> {
        // err inside closure to lazily construct string, should be rare path
        let rmonster = r.monster_data.ok_or_else(|| CardCreationError::MissingMonsterData {
            missing_fields: "atk, def, level, attribute".to_string()
        })?;
        let def = match (&rmonster.linkmarkers, rmonster.def) {
            (Some(l), None) => l.len() as i16,  // take unused DEF values for Link Ratings
            (None, Some(d)) => d,
            (Some(l), Some(d)) => return Err(CardCreationError::ConflictingLinkDefValues {
                link: Some(l.len() as u8),
                def: Some(d)
            }),
            (None, None) => return Err(CardCreationError::ConflictingLinkDefValues { link: None, def: None })
        };

        let flavor = if r.card_type.contains("Normal")  { MonsterFlavor::Normal }
            else if r.card_type.contains("Ritual")  { MonsterFlavor::Ritual }
            else if r.card_type.contains("Fusion")  { MonsterFlavor::Fusion }
            else if r.card_type.contains("Synchro") { MonsterFlavor::Synchro }
            else if r.card_type.contains("XYZ")     { MonsterFlavor::Xyz }
            else if r.card_type.contains("Link")    { MonsterFlavor::Link(rmonster.linkmarkers
                .ok_or_else(|| CardCreationError::MissingMonsterData { missing_fields: "link markers".to_string() })?
            )}
            else { MonsterFlavor::Effect };

        let (desc, pendulum) = if r.card_type.contains("Pendulum") {
            let m = rmonster.monster_desc.ok_or_else(|| CardCreationError::MissingMonsterData {
                missing_fields: "monster effect for pendulum monster. Check for missing scale and pendulum effect too".to_string()
            })?;
            let p = Some(Pendulum {
                pend_eff: rmonster.pend_desc.ok_or_else(|| CardCreationError::MissingMonsterData {
                    missing_fields: "pendulum effect. Check for missing scale and monster effect too".to_string()
                })?,
                scale: rmonster.scale.ok_or_else(|| CardCreationError::MissingMonsterData {
                    missing_fields: "scale. Check for missing pendulum and monster effects too".to_string()
                })?
            });
            (m, p)
        }
        else { (r.desc, None) };

        let ctype = CardType::Monster {
            level: rmonster.level,  atk: rmonster.atk,  def,
            flavor,
            attribute: rmonster.attribute,
            mtype: r.race.try_into()?,
            subtypes: SubType::get_all_from_card_type(&r.card_type),
            pendulum
        };

        Ok(YGOCard {
            id: r.id,   name: r.name,
            desc,       ctype,
            img: r.card_images[0].clone()
        })
    }
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
impl TryFrom<response_card::Race> for MonsterType {
    type Error = CardCreationError;

    fn try_from(value: response_card::Race) -> Result<Self, Self::Error> {
        use response_card::Race;
        match value {
            Race::Aqua      => Ok(MonsterType::Aqua),
            Race::Beast     => Ok(MonsterType::Beast),
            Race::BeastWarrior  => Ok(MonsterType::BeastWarrior),
            Race::CreatorGod    => Ok(MonsterType::CreatorGod),
            Race::Cyberse   => Ok(MonsterType::Cyberse),
            Race::Dinosaur  => Ok(MonsterType::Dinosaur),
            Race::DivineBeast   => Ok(MonsterType::DivineBeast),
            Race::Dragon    => Ok(MonsterType::Dragon),
            Race::Fairy     => Ok(MonsterType::Fairy),
            Race::Fiend     => Ok(MonsterType::Fiend),
            Race::Fish      => Ok(MonsterType::Fish),
            Race::Insect    => Ok(MonsterType::Insect),
            Race::Machine   => Ok(MonsterType::Machine),
            Race::Plant     => Ok(MonsterType::Plant),
            Race::Psychic   => Ok(MonsterType::Psychic),
            Race::Pyro      => Ok(MonsterType::Pyro),
            Race::Reptile   => Ok(MonsterType::Reptile),
            Race::Rock      => Ok(MonsterType::Rock),
            Race::SeaSerpent    => Ok(MonsterType::SeaSerpent),
            Race::Spellcaster   => Ok(MonsterType::Spellcaster),
            Race::Thunder   => Ok(MonsterType::Thunder),
            Race::Warrior   => Ok(MonsterType::Warrior),
            Race::WingedBeast   => Ok(MonsterType::WingedBeast),
            Race::Wyrm      => Ok(MonsterType::Wyrm),
            Race::Zombie    => Ok(MonsterType::Zombie),
            _ => Err(CardCreationError::InvalidMonsterType { given: value })
        }
    }
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
impl SubType {
    fn get_all_from_card_type(card_type: &str) -> Rc<[Self]> {
        let mut ret = Vec::with_capacity(6);
        if card_type.contains("Flip")   { ret.push(Self::Flip); }
        if card_type.contains("Tuner")  { ret.push(Self::Tuner); }
        if card_type.contains("Gemini") { ret.push(Self::Gemini); }
        if card_type.contains("Spirit") { ret.push(Self::Spirit); }
        if card_type.contains("Toon")   { ret.push(Self::Toon); }
        if card_type.contains("Union")  { ret.push(Self::Union); }
        ret.into()
    }
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
impl TryFrom<response_card::Race> for SpellType {
    type Error = CardCreationError;

    fn try_from(value: response_card::Race) -> Result<Self, Self::Error> {
        use response_card::Race;
        match value {
            Race::Normal => Ok(SpellType::Normal),
            Race::Equip => Ok(SpellType::Equip),
            Race::Field => Ok(SpellType::Field),
            Race::Continuous => Ok(SpellType::Continuous),
            Race::QuickPlay => Ok(SpellType::QuickPlay),
            Race::Ritual => Ok(SpellType::Ritual),
            _ => Err(CardCreationError::InvalidSpellType { given: value })
        }
    }
}

#[derive(Deserialize, Serialize)]
enum TrapType {
    Normal,
    Continuous,
    Counter
}
impl TryFrom<response_card::Race> for TrapType {
    type Error = CardCreationError;

    fn try_from(value: response_card::Race) -> Result<Self, Self::Error> {
        use response_card::Race;
        match value {
            Race::Normal => Ok(TrapType::Normal),
            Race::Continuous => Ok(TrapType::Continuous),
            Race::Counter => Ok(TrapType::Counter),
            _ => Err(CardCreationError::InvalidTrapType { given: value })
        }
    }
}

#[derive(Debug)]
enum CardCreationError {
    InvalidType { given: InvalidType },
    ConflictingLinkDefValues { link: Option<u8>, def: Option<i16> },
    MissingMonsterData { missing_fields: String },
    InvalidMonsterType { given: response_card::Race },
    InvalidSpellType { given: response_card::Race },
    InvalidTrapType { given: response_card::Race }
}

#[derive(Debug)]
enum InvalidType {
    Skill,
    Token,
    Other
}
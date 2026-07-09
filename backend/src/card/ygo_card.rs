#![allow(unused)]

use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::card::{
    Attribute,
    LinkMarkers,
    ImgLinks,
    response_card
};


#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct YGOCard {
    id: u32,
    name: Rc<str>,
    desc: Rc<str>,
    ctype: CardType,
    img: ImgLinks
}
impl YGOCard {
    pub fn new_from_response(r: response_card::ResponseCard) -> Result<Self, CardCreationError> {
        if r.race == response_card::Race::Other {
            Err(CardCreationError::InvalidType)
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
            else if r.card_type.contains("Xyz")     { MonsterFlavor::Xyz }
            else if r.card_type.contains("Link")    { MonsterFlavor::Link(rmonster.linkmarkers
                .ok_or_else(|| CardCreationError::MissingMonsterData { missing_fields: "link markers".to_string() })?
            )}
            else { MonsterFlavor::Effect };

        let (desc, pendulum) = if r.card_type.contains("Pendulum") {
            let p = Some(Pendulum {
                pend_eff: rmonster.pend_desc.unwrap_or(Rc::from("")),   // could be empty if majespecter
                scale: rmonster.scale.ok_or_else(|| CardCreationError::MissingMonsterData {
                    missing_fields: "scale. Check for missing pendulum and monster effects too".to_string()
                })?
            });

            // monster desc could be empty if majespecter-like
            // (no pend eff so YGOPro forgoes those fields, puts all in "desc" like typical monster)
            let m = rmonster.monster_desc.unwrap_or(r.desc);

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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum CardType {
    Monster {
        atk: i16,
        def: i16,
        level: u8,
        attribute: Attribute,
        mtype: MonsterType,
        flavor: MonsterFlavor,
        subtypes: Rc<[SubType]>,
        pendulum: Option<Pendulum>,
    },
    Spell(SpellType),
    Trap(TrapType)
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum MonsterFlavor {
    Normal,
    Effect,
    Ritual,
    Fusion,
    Synchro,
    Xyz,
    Link(Rc<[LinkMarkers]>)
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Pendulum {
    pend_eff: Rc<str>,
    scale: u8
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
    InvalidType,
    ConflictingLinkDefValues { link: Option<u8>, def: Option<i16> },
    MissingMonsterData { missing_fields: String },
    InvalidMonsterType { given: response_card::Race },
    InvalidSpellType { given: response_card::Race },
    InvalidTrapType { given: response_card::Race }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{
        tests as card_tests,
        response_card::tests::response_json
    };

    fn card_obj(name: &card_tests::ResponseCardName) -> YGOCard {
        use card_tests::ResponseCardName;
        match name {
            ResponseCardName::SummonedSkull => YGOCard {
                id: 70781052,
                name: Rc::from("Summoned Skull"),
                desc: Rc::from("A fiend with dark powers for confusing the enemy. Among the Fiend-Type monsters, this monster boasts considerable force.\n\n(This card is always treated as an \"Archfiend\" card.)"),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/70781052.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/70781052.jpg")
                },
                ctype: CardType::Monster {
                    atk: 2500,  def: 1200,  level: 6,
                    attribute: Attribute::Dark,
                    mtype: MonsterType::Fiend,
                    flavor: MonsterFlavor::Normal,
                    subtypes: Rc::from([]),
                    pendulum: None
                }
            },
            ResponseCardName::TurboTainted  => YGOCard {
                id: 16769305,
                name: Rc::from("Turbo-Tainted Hot Rod GT19"),
                desc: Rc::from("FLIP: You can declare a Level from 1 to 9; this card becomes that Level until the end of this turn.\r\nAfter this card has been flipped face-up, during any Main Phase (Quick Effect): You can target 1 other face-up monster on either field; immediately after this effect resolves, Synchro Summon 1 monster using only this card and that target. You can only use 1 \"Turbo-Tainted Hot Rod GT19\" effect per turn, and only once that turn."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/16769305.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/16769305.jpg")
                },
                ctype: CardType::Monster {
                    atk: 500,   def: 1500,  level: 3,
                    attribute: Attribute::Fire,
                    mtype: MonsterType::Machine,
                    flavor: MonsterFlavor::Effect,
                    subtypes: Rc::from([SubType::Flip, SubType::Tuner]),
                    pendulum: None
                }
            },
            ResponseCardName::Calculator    => YGOCard {
                id: 51196174,
                name: Rc::from("The Calculator"),
                desc: Rc::from("The ATK of this card is the combined Levels of all face-up monsters you control x 300."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/51196174.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/51196174.jpg")
                },
                ctype: CardType::Monster {
                    atk: -1,    def: 0,     level: 2,
                    attribute: Attribute::Light,
                    mtype: MonsterType::Thunder,
                    flavor: MonsterFlavor::Effect,
                    subtypes: Rc::from([]),
                    pendulum: None
                }
            },
            ResponseCardName::Mst       => YGOCard {
                id: 5318639,
                name: Rc::from("Mystical Space Typhoon"),
                desc: Rc::from("Target 1 Spell/Trap on the field; destroy that target."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/5318639.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/5318639.jpg")
                },
                ctype: CardType::Spell(SpellType::QuickPlay)
            },
            ResponseCardName::Solemn    => YGOCard {
                id: 41420027,
                name: Rc::from("Solemn Judgment"),
                desc: Rc::from("When a monster(s) would be Summoned, OR a Spell/Trap Card is activated: Pay half your LP; negate the Summon or activation, and if you do, destroy that card."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/41420027.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/41420027.jpg")
                },
                ctype: CardType::Trap(TrapType::Counter)
            },
            ResponseCardName::Igknight  => YGOCard {
                id: 24131534,
                name: Rc::from("Igknight Squire"),
                desc: Rc::from("''The cold steel armor of this young squire cannot hide the keen, burning mind contained within.''"),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/24131534.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/24131534.jpg")
                },
                ctype: CardType::Monster {
                    atk: 0,     def: 2000,  level: 3,
                    attribute: Attribute::Fire,
                    mtype: MonsterType::Warrior,
                    flavor: MonsterFlavor::Normal,
                    subtypes: Rc::from([]),
                    pendulum: Some(Pendulum {
                        pend_eff: Rc::from("If you have an \"Igknight\" card in your other Pendulum Zone: You can destroy both cards in your Pendulum Zones, and if you do, add 1 FIRE Warrior-Type monster from your Deck or Graveyard to your hand."),
                        scale: 7
                    })
                }
            },
            ResponseCardName::Majespecter   => YGOCard {
                id: 68395509,
                name: Rc::from("Majespecter Crow - Yata"),
                desc: Rc::from("When this card is Normal or Special Summoned: You can add 1 \"Majespecter\" Spell Card from your Deck to your hand. You can only use this effect of \"Majespecter Crow - Yata\" once per turn. Cannot be targeted or destroyed by your opponent's card effects."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/68395509.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/68395509.jpg")
                },
                ctype: CardType::Monster {
                    atk: 1000,  def: 1500,  level: 4,
                    attribute: Attribute::Wind,
                    mtype: MonsterType::Spellcaster,
                    flavor: MonsterFlavor::Effect,
                    subtypes: Rc::from([]),
                    pendulum: Some(Pendulum {
                        pend_eff: Rc::from(""),
                        scale: 5
                    })
                }
            },
            ResponseCardName::HanShi    => YGOCard {
                id: 53270092,
                name: Rc::from("Han-Shi Kyudo Spirit"),
                desc: Rc::from("When this card is Normal Summoned: You can return all cards you control in the same column as the cards in your Pendulum Zones to the hand (including the Pendulum Zone cards themselves), then you can add 1 monster with 2400 ATK/1000 DEF from your Deck to your hand, except \"Han-Shi Kyudo Spirit\". Once per turn, during the End Phase, if this card was Normal Summoned or flipped face-up this turn: Return this card to the hand."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/53270092.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/53270092.jpg")
                },
                ctype: CardType::Monster {
                    atk: 2400,  def: 1000,  level: 5,
                    attribute: Attribute::Wind,
                    mtype: MonsterType::Warrior,
                    flavor: MonsterFlavor::Effect,
                    subtypes: Rc::from([SubType::Spirit]),
                    pendulum: Some(Pendulum {
                        pend_eff: Rc::from("If a monster(s) is Pendulum Summoned: Return this card to the hand."),
                        scale: 9
                    })
                }
            },
            ResponseCardName::ClearWingFast => YGOCard {
                id: 90036274,
                name: Rc::from("Clear Wing Fast Dragon"),
                desc: Rc::from("1 Tuner + 1+ non-Tuner WIND monsters\r\n(Quick Effect): You can target 1 face-up monster your opponent controls that was Special Summoned from the Extra Deck; until the end of this turn, change its ATK to 0, also negate that face-up monster's effects. You can only use this effect of \"Clear Wing Fast Dragon\" once per turn. If this card in the Monster Zone is destroyed by battle or card effect: You can place this card in your Pendulum Zone."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/90036274.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/90036274.jpg")
                },
                ctype: CardType::Monster {
                    atk: 2500,  def: 2000,  level: 7,
                    attribute: Attribute::Wind,
                    mtype: MonsterType::Dragon,
                    flavor: MonsterFlavor::Synchro,
                    subtypes: Rc::from([]),
                    pendulum: Some(Pendulum {
                        pend_eff: Rc::from("You can send 1 face-up \"Speedroid\" Tuner and 1 face-up non-Tuner monster you control to the GY, whose total Levels equal 7; Special Summon this card from your Pendulum Zone. You can only use this effect of \"Clear Wing Fast Dragon\" once per turn."),
                        scale: 4
                    })
                }
            },
            ResponseCardName::Tornado   => YGOCard {
                id: 6983839,
                name: Rc::from("Tornado Dragon"),
                desc: Rc::from("2 Level 4 monsters\nOnce per turn (Quick Effect): You can detach 1 material from this card, then target 1 Spell/Trap on the field; destroy it."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/6983839.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/6983839.jpg")
                },
                ctype: CardType::Monster {
                    atk: 2100,  def: 2000,  level: 4,
                    attribute: Attribute::Wind,
                    mtype: MonsterType::Wyrm,
                    flavor: MonsterFlavor::Xyz,
                    subtypes: Rc::from([]),
                    pendulum: None
                }
            },
            ResponseCardName::DecodeTalker  => YGOCard {
                id: 1861629,
                name: Rc::from("Decode Talker"),
                desc: Rc::from("2+ Effect Monsters\r\nGains 500 ATK for each monster it points to. When your opponent activates a card or effect that targets a card(s) you control (Quick Effect): You can Tribute 1 monster this card points to; negate the activation, and if you do, destroy that card."),
                img: ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/1861629.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/1861629.jpg")
                },
                ctype: CardType::Monster {
                    atk: 2300,  def: 3,     level: 0,
                    attribute: Attribute::Dark,
                    mtype: MonsterType::Cyberse,
                    flavor: MonsterFlavor::Link(Rc::from([LinkMarkers::Top, LinkMarkers::BottomLeft, LinkMarkers::BottomRight])),
                    subtypes: Rc::from([]),
                    pendulum: None
                }
            },
        }
    }

    fn test_create_general(name: card_tests::ResponseCardName) {
        let rjson = response_json(&name).data[0].clone();
        let card = YGOCard::new_from_response(rjson)
            .expect("Should be able to create card from hard-coded valid response object");

        let ans = card_obj(&name);
        assert_eq!(card, ans);
    }

    #[test]
    fn create_normal_monster() { test_create_general(card_tests::ResponseCardName::SummonedSkull); }
    
    #[test]
    fn create_effect_monster() { test_create_general(card_tests::ResponseCardName::TurboTainted); }

    #[test]
    fn create_question_atk() { test_create_general(card_tests::ResponseCardName::Calculator); }
    
    #[test]
    fn create_spell() { test_create_general(card_tests::ResponseCardName::Mst); }

    #[test]
    fn create_trap() { test_create_general(card_tests::ResponseCardName::Solemn); }

    #[test]
    fn create_igknight() { test_create_general(card_tests::ResponseCardName::Igknight); }

    #[test]
    fn create_majespecter() { test_create_general(card_tests::ResponseCardName::Majespecter); }

    #[test]
    fn create_pendulum() { test_create_general(card_tests::ResponseCardName::HanShi); }

    #[test]
    fn create_ed_pendulum() { test_create_general(card_tests::ResponseCardName::ClearWingFast); }

    #[test]
    fn create_xyz() { test_create_general(card_tests::ResponseCardName::Tornado); }

    #[test]
    fn create_link() { test_create_general(card_tests::ResponseCardName::DecodeTalker); }
}
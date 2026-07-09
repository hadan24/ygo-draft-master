use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::card::{
    Attribute,
    LinkMarkers,
    ImgLinks
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct YGOProResponse{
    pub(super) data: Rc<[ResponseCard]>
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(super) struct ResponseCard {
    pub(super) id: u32,
    pub(super) name: Rc<str>,
    #[serde(alias="humanReadableCardType")]
    pub(super) card_type: Rc<str>,
    pub(super) desc: Rc<str>,
    pub(super) race: Race,
    pub(super) card_images: Rc<[ImgLinks]>,

    #[serde(flatten)]
    pub(super) monster_data: Option<MonsterData>
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(super) struct MonsterData {
    pub(super) atk: i16,
    pub(super) def: Option<i16>,
    pub(super) level: u8,
    pub(super) attribute: Attribute,
    pub(super) pend_desc: Option<Rc<str>>,
    pub(super) monster_desc: Option<Rc<str>>,
    pub(super) scale: Option<u8>,
    pub(super) linkmarkers: Option<Rc<[LinkMarkers]>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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


#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::card::tests as card_tests;

    pub(in crate::card) fn response_json(name: &card_tests::ResponseCardName) -> YGOProResponse {
        use card_tests::ResponseCardName;
        match name {
            ResponseCardName::SummonedSkull => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 70781052,
                name: Rc::from("Summoned Skull"),
                card_type: Rc::from("Normal Monster"),
                desc: Rc::from("A fiend with dark powers for confusing the enemy. Among the Fiend-Type monsters, this monster boasts considerable force.\n\n(This card is always treated as an \"Archfiend\" card.)"),
                race: Race::Fiend,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/70781052.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/70781052.jpg")
                    },
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/70781053.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/70781053.jpg")
                    },
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/70781054.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/70781054.jpg")
                    },
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/70781055.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/70781055.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 2500,
                    def: Some(1200),
                    level: 6,
                    attribute: Attribute::Dark,
                    pend_desc: None,
                    monster_desc: None,
                    scale: None,
                    linkmarkers: None
                })
            }])},
            ResponseCardName::TurboTainted  => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 16769305,
                name: Rc::from("Turbo-Tainted Hot Rod GT19"),
                card_type: Rc::from("Flip Tuner Effect Monster"),
                desc: Rc::from("FLIP: You can declare a Level from 1 to 9; this card becomes that Level until the end of this turn.\r\nAfter this card has been flipped face-up, during any Main Phase (Quick Effect): You can target 1 other face-up monster on either field; immediately after this effect resolves, Synchro Summon 1 monster using only this card and that target. You can only use 1 \"Turbo-Tainted Hot Rod GT19\" effect per turn, and only once that turn."),
                race: Race::Machine,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/16769305.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/16769305.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 500,
                    def: Some(1500),
                    level: 3,
                    attribute: Attribute::Fire,
                    pend_desc: None,
                    monster_desc: None,
                    scale: None,
                    linkmarkers: None
                })
            }])},
            ResponseCardName::Calculator    => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 51196174,
                name: Rc::from("The Calculator"),
                card_type: Rc::from("Effect Monster"),
                desc: Rc::from("The ATK of this card is the combined Levels of all face-up monsters you control x 300."),
                race: Race::Thunder,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/51196174.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/51196174.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: -1,
                    def: Some(0),
                    level: 2,
                    attribute: Attribute::Light,
                    pend_desc: None,
                    monster_desc: None,
                    scale: None,
                    linkmarkers: None
                })
            }])},
            ResponseCardName::Mst       => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 5318639,
                name: Rc::from("Mystical Space Typhoon"),
                card_type: Rc::from("Quick-Play Spell"),
                desc: Rc::from("Target 1 Spell/Trap on the field; destroy that target."),
                race: Race::QuickPlay,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/5318639.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/5318639.jpg")
                    }
                ]),
                monster_data: None
            }])},
            ResponseCardName::Solemn    => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 41420027,
                name: Rc::from("Solemn Judgment"),
                card_type: Rc::from("Counter Trap"),
                desc: Rc::from("When a monster(s) would be Summoned, OR a Spell/Trap Card is activated: Pay half your LP; negate the Summon or activation, and if you do, destroy that card."),
                race: Race::Counter,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/41420027.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/41420027.jpg")
                    }
                ]),
                monster_data: None
            }])},
            ResponseCardName::Igknight  => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 24131534,
                name: Rc::from("Igknight Squire"),
                card_type: Rc::from("Pendulum Normal Monster"),
                desc: Rc::from("[ Pendulum Effect ] \nIf you have an \"Igknight\" card in your other Pendulum Zone: You can destroy both cards in your Pendulum Zones, and if you do, add 1 FIRE Warrior-Type monster from your Deck or Graveyard to your hand.\n\n[ Monster Effect ] \n''The cold steel armor of this young squire cannot hide the keen, burning mind contained within.''"),
                race: Race::Warrior,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/24131534.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/24131534.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 0,
                    def: Some(2000),
                    level: 3,
                    attribute: Attribute::Fire,
                    pend_desc: Some(Rc::from("If you have an \"Igknight\" card in your other Pendulum Zone: You can destroy both cards in your Pendulum Zones, and if you do, add 1 FIRE Warrior-Type monster from your Deck or Graveyard to your hand.")),
                    monster_desc: Some(Rc::from("''The cold steel armor of this young squire cannot hide the keen, burning mind contained within.''")),
                    scale: Some(7),
                    linkmarkers: None
                })
            }])},
            ResponseCardName::Majespecter   => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 68395509,
                name: Rc::from("Majespecter Crow - Yata"),
                card_type: Rc::from("Pendulum Effect Monster"),
                desc: Rc::from("When this card is Normal or Special Summoned: You can add 1 \"Majespecter\" Spell Card from your Deck to your hand. You can only use this effect of \"Majespecter Crow - Yata\" once per turn. Cannot be targeted or destroyed by your opponent's card effects."),
                race: Race::Spellcaster,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/68395509.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/68395509.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 1000,
                    def: Some(1500),
                    level: 4,
                    attribute: Attribute::Wind,
                    pend_desc: None,
                    monster_desc: None,
                    scale: Some(5),
                    linkmarkers: None
                })
            }])},
            ResponseCardName::HanShi    => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 53270092,
                name: Rc::from("Han-Shi Kyudo Spirit"),
                card_type: Rc::from("Pendulum Spirit Effect Monster"),
                desc: Rc::from("[ Pendulum Effect ] \nIf a monster(s) is Pendulum Summoned: Return this card to the hand.\n\n[ Monster Effect ] \nWhen this card is Normal Summoned: You can return all cards you control in the same column as the cards in your Pendulum Zones to the hand (including the Pendulum Zone cards themselves), then you can add 1 monster with 2400 ATK/1000 DEF from your Deck to your hand, except \"Han-Shi Kyudo Spirit\". Once per turn, during the End Phase, if this card was Normal Summoned or flipped face-up this turn: Return this card to the hand."),
                race: Race::Warrior,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/53270092.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/53270092.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 2400,
                    def: Some(1000),
                    level: 5,
                    attribute: Attribute::Wind,
                    pend_desc: Some(Rc::from("If a monster(s) is Pendulum Summoned: Return this card to the hand.")),
                    monster_desc: Some(Rc::from("When this card is Normal Summoned: You can return all cards you control in the same column as the cards in your Pendulum Zones to the hand (including the Pendulum Zone cards themselves), then you can add 1 monster with 2400 ATK/1000 DEF from your Deck to your hand, except \"Han-Shi Kyudo Spirit\". Once per turn, during the End Phase, if this card was Normal Summoned or flipped face-up this turn: Return this card to the hand.")),
                    scale: Some(9),
                    linkmarkers: None
                })
            }])},
            ResponseCardName::ClearWingFast => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 90036274,
                name: Rc::from("Clear Wing Fast Dragon"),
                card_type: Rc::from("Synchro Pendulum Effect Monster"),
                desc: Rc::from("[ Pendulum Effect ] \nYou can send 1 face-up \"Speedroid\" Tuner and 1 face-up non-Tuner monster you control to the GY, whose total Levels equal 7; Special Summon this card from your Pendulum Zone. You can only use this effect of \"Clear Wing Fast Dragon\" once per turn.\n\n[ Monster Effect ] \n1 Tuner + 1+ non-Tuner WIND monsters\r\n(Quick Effect): You can target 1 face-up monster your opponent controls that was Special Summoned from the Extra Deck; until the end of this turn, change its ATK to 0, also negate that face-up monster's effects. You can only use this effect of \"Clear Wing Fast Dragon\" once per turn. If this card in the Monster Zone is destroyed by battle or card effect: You can place this card in your Pendulum Zone."),
                race: Race::Dragon,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/90036274.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/90036274.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 2500,
                    def: Some(2000),
                    level: 7,
                    attribute: Attribute::Wind,
                    pend_desc: Some(Rc::from("You can send 1 face-up \"Speedroid\" Tuner and 1 face-up non-Tuner monster you control to the GY, whose total Levels equal 7; Special Summon this card from your Pendulum Zone. You can only use this effect of \"Clear Wing Fast Dragon\" once per turn.")),
                    monster_desc: Some(Rc::from("1 Tuner + 1+ non-Tuner WIND monsters\r\n(Quick Effect): You can target 1 face-up monster your opponent controls that was Special Summoned from the Extra Deck; until the end of this turn, change its ATK to 0, also negate that face-up monster's effects. You can only use this effect of \"Clear Wing Fast Dragon\" once per turn. If this card in the Monster Zone is destroyed by battle or card effect: You can place this card in your Pendulum Zone.")),
                    scale: Some(4),
                    linkmarkers: None
                })
            }])},
            ResponseCardName::Tornado   => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 6983839,
                name: Rc::from("Tornado Dragon"),
                card_type: Rc::from("Xyz Effect Monster"),
                desc: Rc::from("2 Level 4 monsters\nOnce per turn (Quick Effect): You can detach 1 material from this card, then target 1 Spell/Trap on the field; destroy it."),
                race: Race::Wyrm,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/6983839.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/6983839.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 2100,
                    def: Some(2000),
                    level: 4,
                    attribute: Attribute::Wind,
                    pend_desc: None,
                    monster_desc: None,
                    scale: None,
                    linkmarkers: None
                })
            }])},
            ResponseCardName::DecodeTalker  => YGOProResponse { data: Rc::from([ ResponseCard {
                id: 1861629,
                name: Rc::from("Decode Talker"),
                card_type: Rc::from("Link Effect Monster"),
                desc: Rc::from("2+ Effect Monsters\r\nGains 500 ATK for each monster it points to. When your opponent activates a card or effect that targets a card(s) you control (Quick Effect): You can Tribute 1 monster this card points to; negate the activation, and if you do, destroy that card."),
                race: Race::Cyberse,
                card_images: Rc::from([
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/1861629.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/1861629.jpg")
                    },
                    ImgLinks {
                        small: Rc::from("https://images.ygoprodeck.com/images/cards_small/1861630.jpg"),
                        cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/1861630.jpg")
                    }
                ]),
                monster_data: Some(MonsterData {
                    atk: 2300,
                    def: None,
                    level: 0,
                    attribute: Attribute::Dark,
                    pend_desc: None,
                    monster_desc: None,
                    scale: None,
                    linkmarkers: Some(Rc::from([
                        LinkMarkers::Top,
                        LinkMarkers::BottomLeft,
                        LinkMarkers::BottomRight
                    ]))
                })
            }])},
        }
    }

    fn test_de_general(name: card_tests::ResponseCardName) {
        let rstr = card_tests::response_str(&name);
        let rbody: reqwest::blocking::Response = http::Response::new(rstr).into();
        let rjson = rbody.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");

        let ans = response_json(&name);
        assert_eq!(rjson, ans)
    }

    #[test]
    fn de_normal_monster() { test_de_general(card_tests::ResponseCardName::SummonedSkull); }
    
    #[test]
    fn de_effect_monster() { test_de_general(card_tests::ResponseCardName::TurboTainted); }

    #[test]
    fn de_question_atk() { test_de_general(card_tests::ResponseCardName::Calculator); }
    
    #[test]
    fn de_spell() { test_de_general(card_tests::ResponseCardName::Mst); }

    #[test]
    fn de_trap() { test_de_general(card_tests::ResponseCardName::Solemn); }

    #[test]
    fn de_igknight() { test_de_general(card_tests::ResponseCardName::Igknight); }

    #[test]
    fn de_majespecter() { test_de_general(card_tests::ResponseCardName::Majespecter); }

    #[test]
    fn de_pendulum() { test_de_general(card_tests::ResponseCardName::HanShi); }

    #[test]
    fn de_ed_pendulum() { test_de_general(card_tests::ResponseCardName::ClearWingFast); }

    #[test]
    fn de_xyz() { test_de_general(card_tests::ResponseCardName::Tornado); }

    #[test]
    fn de_link() { test_de_general(card_tests::ResponseCardName::DecodeTalker); }
}
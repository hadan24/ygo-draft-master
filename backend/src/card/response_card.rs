use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::card::{
    Attribute,
    LinkMarkers,
    ImgLinks
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct YGOProResponse{
    data: Vec<ResponseCard>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(super) struct MonsterData {
    pub(super) atk: i16,
    pub(super) def: Option<i16>,
    pub(super) level: u8,
    pub(super) attribute: Attribute,
    pub(super) pend_desc: Option<Rc<str>>,
    pub(super) monster_desc: Option<Rc<str>>,
    pub(super) scale: Option<u8>,
    pub(super) linkmarkers: Option<Vec<LinkMarkers>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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


mod test {
    use super::*;
    use reqwest::blocking::Response as ReqwestResponse;
    use crate::card::test::*;

    #[test]
    fn de_normal_monster() {
        let r: ReqwestResponse = http::Response::new(SUMMONED_SKULL).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
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
        }]};

        assert_eq!(r, ans)
    }
    
    #[test]
    fn de_effect_monster() {
        let r: ReqwestResponse = http::Response::new(TURBO_TAINTED).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
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
        }]};

        assert_eq!(r, ans)
    }

    #[test]
    fn de_question_atk() {
        let r: ReqwestResponse = http::Response::new(CALCULATOR).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
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
        }]};

        assert_eq!(r, ans)
    }
    
    #[test]
    fn de_spell() {
        let r: ReqwestResponse = http::Response::new(MST).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
            id: 5318639,
            name: Rc::from("Mystical Space Typhoon"),
            card_type: Rc::from("Spell Card"),
            desc: Rc::from("Target 1 Spell/Trap on the field; destroy that target."),
            race: Race::QuickPlay,
            card_images: Rc::from([
                ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/5318639.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/5318639.jpg")
                }
            ]),
            monster_data: None
        }]};

        assert_eq!(r, ans)
    }

    #[test]
    fn de_trap() {
        let r: ReqwestResponse = http::Response::new(SOLEMN).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
            id: 41420027,
            name: Rc::from("Solemn Judgment"),
            card_type: Rc::from("Trap Card"),
            desc: Rc::from("When a monster(s) would be Summoned, OR a Spell/Trap Card is activated: Pay half your LP; negate the Summon or activation, and if you do, destroy that card."),
            race: Race::Counter,
            card_images: Rc::from([
                ImgLinks {
                    small: Rc::from("https://images.ygoprodeck.com/images/cards_small/41420027.jpg"),
                    cropped: Rc::from("https://images.ygoprodeck.com/images/cards_cropped/41420027.jpg")
                }
            ]),
            monster_data: None
        }]};

        assert_eq!(r, ans)
    }

    #[test]
    fn de_pendulum() {
        let r: ReqwestResponse = http::Response::new(HAN_SHI).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
            id: 53270092,
            name: Rc::from("Han-Shi Kyudo Spirit"),
            card_type: Rc::from("Spirit Monster"),
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
        }]};

        assert_eq!(r, ans)
    }

    #[test]
    fn de_ed_pendulum() {
        let r: ReqwestResponse = http::Response::new(CLEAR_WING_FAST).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
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
        }]};

        assert_eq!(r, ans)
    }

    #[test]
    fn de_xyz() {
        let r: ReqwestResponse = http::Response::new(TORNADO).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
            id: 6983839,
            name: Rc::from("Tornado Dragon"),
            card_type: Rc::from("XYZ Monster"),
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
        }]};

        assert_eq!(r, ans)
    }

    #[test]
    fn de_link() {
        let r: ReqwestResponse = http::Response::new(DECODE_TALKER).into();
        let r = r.json::<YGOProResponse>()
            .expect("Should be able to decode valid JSON copy-pasted from API response");
        let ans = YGOProResponse { data: vec![ ResponseCard {
            id: 1861629,
            name: Rc::from("Decode Talker"),
            card_type: Rc::from("Link Monster"),
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
                linkmarkers: Some(vec![
                    LinkMarkers::Top,
                    LinkMarkers::BottomLeft,
                    LinkMarkers::BottomRight
                ])
            })
        }]};

        assert_eq!(r, ans)
    }
}
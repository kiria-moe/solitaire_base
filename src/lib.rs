pub mod card;
pub mod board_slot;
pub mod move_action;
pub mod index;

use card::{Card, DragonCard, NumberCard};
use board_slot::{BoardOut, BoardSpare};
use move_action::MoveAction;

use std::fmt::Display;
use std::mem;
use serde_json::json;
use crate::index::{ALL_SLOTS, Location, Slot};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub flower: bool,
    pub spare: [BoardSpare; 3],
    pub out: BoardOut,
    pub tray: [Vec<Card>; 8],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"+--+--+--+-----+--+--+--+")?;
        (0..3).for_each(|i| write!(f,"|{}", self.spare[i]).unwrap());
        write!(f,"| {} |", if self.flower { "F L" } else { "   " })?;
        writeln!(f,"G{}|B{}|R{}|", self.out.bamboo, self.out.characters, self.out.coin)?;
        writeln!(f,"+--+--+--+-----+--+--+--+")?;
        let height = self.tray.iter().map(|x| x.len()).max().unwrap_or(0);
        for i in 0..height {
            for stack in self.tray.iter() {
                write!(f, " {}", if let Some(c) = stack.get(i) { std::borrow::Cow::Owned(format!("{c}")) } else { std::borrow::Cow::Borrowed("  ") })?;
            }
            writeln!(f, " ")?;
        }
        Ok(())
    }
}

impl From<serde_json::Value> for Board {
    fn from(v: serde_json::Value) -> Self {
        let flower = v["flower"].as_bool().unwrap();
        let spare: [BoardSpare; 3] = v["spare"].as_array().unwrap().iter().map(|x| match x["type"].as_str().unwrap(){
            "empty" => BoardSpare::Empty,
            "collected" => BoardSpare::Collected,
            "special" => BoardSpare::Card(match x["color"].as_str().unwrap() {
                "dragon_green" => Card::Dragon(DragonCard::Green),
                "dragon_black" => Card::Dragon(DragonCard::White),
                "dragon_red" => Card::Dragon(DragonCard::Red),
                "flower" => Card::Flower,
                _ => unreachable!(),
            }),
            "number" => BoardSpare::Card(match x["color"].as_str().unwrap() {
                "bamboo" => Card::Number(NumberCard::Bamboo, x["value"].as_u64().unwrap() as u8),
                "characters" => Card::Number(NumberCard::Characters, x["value"].as_u64().unwrap() as u8),
                "coins" => Card::Number(NumberCard::Coin, x["value"].as_u64().unwrap() as u8),
                _ => unreachable!(),
            }),
            _ => unreachable!(),
        }).collect::<Vec<BoardSpare>>().try_into().unwrap();
        let out = BoardOut {
            bamboo: v["out"]["bamboo"].as_u64().unwrap() as u8,
            characters: v["out"]["char"].as_u64().unwrap() as u8,
            coin: v["out"]["coin"].as_u64().unwrap() as u8,
        };
        let tray: [Vec<Card>; 8] = v["tray"].as_array().unwrap().iter().map(|x| x.as_array().unwrap().iter().map(|y| match y["type"].as_str().unwrap() {
            "special" => Card::Dragon(match y["color"].as_str().unwrap() {
                "dragon_green" => DragonCard::Green,
                "dragon_white" => DragonCard::White,
                "dragon_red" => DragonCard::Red,
                _ => unreachable!(),
            }),
            "number" => Card::Number(match y["color"].as_str().unwrap() {
                "bamboo" => NumberCard::Bamboo,
                "characters" => NumberCard::Characters,
                "coins" => NumberCard::Coin,
                _ => unreachable!(),
            }, y["value"].as_u64().unwrap() as u8),
            "flower" => Card::Flower,
            _ => unreachable!()
        }).collect::<Vec<Card>>()).collect::<Vec<Vec<Card>>>().try_into().unwrap();
        Board { flower, spare, out, tray }
    }
}

impl From<&Board> for serde_json::Value {
    fn from(value: &Board) -> Self {
        json!({
            "flower": value.flower,
            "out": {
                "bamboo": value.out.bamboo,
                "char": value.out.characters,
                "coin": value.out.coin,
            },
            "spare": value.spare.iter().map(|x| match x {
                BoardSpare::Empty => json!({"type": "empty"}),
                BoardSpare::Collected => json!({"type": "collected"}),
                BoardSpare::Card(c) => match c {
                    Card::Flower => json!({"type": "flower"}),
                    Card::Dragon(DragonCard::Green) => json!({"type": "special", "color": "dragon_green"}),
                    Card::Dragon(DragonCard::White) => json!({"type": "special", "color": "dragon_white"}),
                    Card::Dragon(DragonCard::Red) => json!({"type": "special", "color": "dragon_red"}),
                    Card::Number(NumberCard::Bamboo, n) => json!({"type": "number", "color": "bamboo", "value": n}),
                    Card::Number(NumberCard::Characters, n) => json!({"type": "number", "color": "characters", "value": n}),
                    Card::Number(NumberCard::Coin, n) => json!({"type": "number", "color": "coins", "value": n}),
                }
            }).collect::<Vec<serde_json::Value>>(),
            "tray": value.tray.iter().map(|x| x.iter().map(|y| match y {
                Card::Flower => json!({"type": "flower"}),
                Card::Dragon(DragonCard::Green) => json!({"type": "special", "color": "dragon_green"}),
                Card::Dragon(DragonCard::White) => json!({"type": "special", "color": "dragon_white"}),
                Card::Dragon(DragonCard::Red) => json!({"type": "special", "color": "dragon_red"}),
                Card::Number(NumberCard::Bamboo, n) => json!({"type": "number", "color": "bamboo", "value": n}),
                Card::Number(NumberCard::Characters, n) => json!({"type": "number", "color": "characters", "value": n}),
                Card::Number(NumberCard::Coin, n) => json!({"type": "number", "color": "coins", "value": n}),
            }).collect::<Vec<serde_json::Value>>()).map(|x| x.into()).collect::<Vec<serde_json::Value>>(),
        })
    }
}

impl std::ops::Index<Location> for Board {
    type Output = Card;
    fn index(&self, location: Location) -> &Self::Output {
        use Location as L;
        match location {
            L::Spare(index) => match self.spare[index as usize] {
                BoardSpare::Card(ref c) => c,
                _ => panic!("Slot collected or empty"),
            },
            L::Tray(x, y) => &self.tray[x as usize][y as usize],
        }
    }
}

impl Board {
    pub fn new_random() -> Self {
        let mut cards = vec![Card::Flower];
        (0..4).for_each(|_| {
            cards.push(Card::Dragon(DragonCard::Green));
            cards.push(Card::Dragon(DragonCard::White));
            cards.push(Card::Dragon(DragonCard::Red));
        });
        (1..=9).for_each(|n| {
            cards.push(Card::Number(NumberCard::Bamboo, n));
            cards.push(Card::Number(NumberCard::Characters, n));
            cards.push(Card::Number(NumberCard::Coin, n));
        });
        use rand::prelude::*;
        cards.shuffle(&mut rand::thread_rng());
        let mut ret = Self {
            flower: false,
            spare: [BoardSpare::Empty, BoardSpare::Empty, BoardSpare::Empty],
            out: BoardOut { bamboo: 0, characters: 0, coin: 0 },
            tray: cards.chunks_exact(5).map(|x|x.to_vec()).collect::<Vec<Vec<Card>>>().try_into().unwrap(),
        };
        ret.simplify();
        ret
    }
    pub fn move_cards(&mut self, action: MoveAction) -> bool {
        use MoveAction as MA;
        match action {
            MA::CollectDragon(dragon) => {
                if let Some(target_index) = self.dragon_collectable(dragon) {
                    self.spare[target_index as usize] = BoardSpare::Collected;
                    ALL_SLOTS.iter().for_each(|slot| {
                        if let Some(Card::Dragon(d)) = self.last(*slot) {
                            if d == dragon {
                                self.pop(*slot);
                            }
                        }
                    });
                    self.simplify();
                    true
                } else {
                    false
                }
            }
            MA::Move(source, cnt, target) => {
                (0..cnt).map(|_| self.pop(source).unwrap()).collect::<Vec<Card>>()
                    .iter().rev().for_each(|c| self.push(target, *c));
                true
            }
        }
    }
/*    pub fn neighbors(&self) -> Vec<(MoveAction, Board)> {
        let mut ret = vec![];
        let mut new_board = self.clone();
        //collect dragon
        for dragon in [DragonCard::Red, DragonCard::White, DragonCard::Green] {
            if new_board.move_cards(MoveAction::CollectDragon(dragon)).is_ok() {
                let x = mem::replace(&mut new_board, self.clone());
                ret.push((MoveAction::CollectDragon(dragon), x));
            }
        }

        //spare to tray
        for source_index in self.spare.iter().enumerate().filter_map(|(i, x)| if let BoardSpare::Card(_) = x { Some(i) } else { None }) {
            for target_index in 0..self.tray.iter().len() {
                if new_board.move_cards(MoveAction::MoveSpareToTray(source_index as u8, target_index as u8)).is_ok() {
                    let x = mem::replace(&mut new_board, self.clone());
                    ret.push((MoveAction::MoveSpareToTray(source_index as u8, target_index as u8), x));
                }
            }
        }

        //tray to tray
        for (source_index, source_stack) in self.tray.iter().enumerate().filter(|(_, x)| !x.is_empty()) {
            for (target_index, _) in self.tray.iter().enumerate().filter(|(i, _)| *i != source_index) {
                for num in 1..=source_stack.len() {
                    if new_board.move_cards(MoveAction::MoveTrayToTray(source_index as u8, target_index as u8, num as u8)).is_ok() {
                        let x = mem::replace(&mut new_board, self.clone());
                        ret.push((MoveAction::MoveTrayToTray(source_index as u8, target_index as u8, num as u8), x));
                    } else {
                        break;
                    }
                }
            }
        }

        if !ret.is_empty() {
            return ret;
        }

        //tray to spare
        if let Some((target_index, _)) = self.spare.iter().enumerate().find(|(_, x)| matches!(x, BoardSpare::Empty)) {
            for (source_index, _) in self.tray.iter().enumerate().filter(|(_, x)| !x.is_empty()) {
                if new_board.move_cards(MoveAction::MoveTrayToSpare(source_index as u8, target_index as u8)).is_ok() {
                    let x = mem::replace(&mut new_board, self.clone());
                    ret.push((MoveAction::MoveTrayToSpare(source_index as u8, target_index as u8), x));
                }
            }
        }
        ret
    }*/
    pub fn simplify(&mut self) {
        impl BoardOut {
            fn get_board_out(&mut self, c: NumberCard) -> &mut u8 {
                match c {
                    NumberCard::Bamboo => &mut self.bamboo,
                    NumberCard::Characters => &mut self.characters,
                    NumberCard::Coin => &mut self.coin,
                }
            }
        }

        let mut moved = true;
        while moved {
            moved = false;
            for slot in ALL_SLOTS.iter() {
                match self.last(*slot) {
                    Some(Card::Flower) => {
                        self.flower = true;
                        self.pop(*slot);
                        moved = true;
                    }
                    Some(Card::Number(c, 1)) => {
                        *self.out.get_board_out(c) = 1;
                        self.pop(*slot);
                        moved = true;
                    }
                    Some(Card::Number(c, 2)) if *self.out.get_board_out(c) == 1 => {
                        *self.out.get_board_out(c) = 2;
                        self.pop(*slot);
                        moved = true;
                    }
                    Some(Card::Number(c, n)) if self.out.bamboo + 1 >= n
                        && self.out.characters + 1 >= n
                        && self.out.coin + 1 >= n => {
                        *self.out.get_board_out(c) = n;
                        self.pop(*slot);
                        moved = true;
                    }
                    _ => {},
                }
            }
        }
    }
    pub fn len(&self, slot: Slot) -> usize {
        if let Slot::Tray(index) = slot {
            self.tray[index as usize].len()
        } else {
            panic!("Only tray has length");
        }
    }
    pub fn get(&self, location: Location) -> Option<Card> {
        match location {
            Location::Spare(index) => match self.spare[index as usize] {
                BoardSpare::Card(c) => Some(c),
                _ => None,
            },
            Location::Tray(x, y) => self.tray[x as usize].get(y as usize).copied(),
        }
    }
    pub fn last(&self, slot: Slot) -> Option<Card> {
        match slot {
            Slot::Spare(index) => self.get(Location::Spare(index)),
            Slot::Tray(index) => self.tray[index as usize].last().copied(),
        }
    }
    pub fn remove(&mut self, location: Location) -> Card {
        match location {
            Location::Spare(index) => {
                if let BoardSpare::Card(c) = self.spare[index as usize] {
                    c
                } else {
                    panic!("Slot collected or empty");
                }
            },
            Location::Tray(x, y) => {
                self.tray[x as usize].remove(y as usize)
            },
        }
    }
    pub fn pop(&mut self, slot: Slot) -> Option<Card> {
        match slot {
            Slot::Spare(index) => match self.spare[index as usize] {
                BoardSpare::Card(c) => {
                    self.spare[index as usize] = BoardSpare::Empty;
                    Some(c)
                },
                _ => None,
            },
            Slot::Tray(index) => self.tray[index as usize].pop(),
        }
    }
    pub fn push(&mut self, slot: Slot, card: Card) {
        match slot {
            Slot::Spare(index) => {
                if let BoardSpare::Empty = self.spare[index as usize] {
                    self.spare[index as usize] = BoardSpare::Card(card);
                } else {
                    panic!("Slot is not empty");
                }
            },
            Slot::Tray(index) => {
                self.tray[index as usize].push(card);
            },
        }
    }
    pub fn appendable(&self, slot: Slot, card: Card) -> bool {
        match slot {
            Slot::Spare(index) => matches!(self.spare[index as usize], BoardSpare::Empty),
            Slot::Tray(index) => self.last(Slot::Tray(index))
                .map_or(true, |c| card.can_stack_onto(&c)),
        }
    }
    pub fn dragon_collectable(&self, color: DragonCard) -> Option<u8> {
        self.spare.iter().position(|x| match x {
            BoardSpare::Empty => true,
            BoardSpare::Card(Card::Dragon(d)) if *d == color => true,
            _ => false,
        }).map(|x| x as u8)
            .and_then(|i| {
                let spare_count = (0..3).filter(|x|
                    self.get(Location::Spare(*x))
                        .map_or(false, |c| matches!(c, Card::Dragon(d) if d == color))).count();
                let tray_count = (0..8).filter(|x|
                    self.last(Slot::Tray(*x))
                        .map_or(false, |c| matches!(c, Card::Dragon(d) if d == color))).count();
                (spare_count + tray_count == 4).then_some(i)
            })
    }
}
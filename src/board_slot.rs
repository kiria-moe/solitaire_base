use std::fmt::Display;
use crate::card::Card;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BoardOut {
    pub bamboo: u8,
    pub characters: u8,
    pub coin: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoardSpare {
    Collected,
    Card(Card),
    Empty,
}

impl Display for BoardSpare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardSpare::Collected => write!(f, "CO"),
            BoardSpare::Card(c) => write!(f, "{}", c),
            BoardSpare::Empty => write!(f, "  "),
        }
    }

}
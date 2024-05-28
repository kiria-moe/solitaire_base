use std::fmt::Display;
use crate::Board;
use crate::card::DragonCard;

#[derive(Debug, Copy, Clone)]
pub enum MoveAction {
    CollectDragon(DragonCard),
    MoveTrayToSpare(u8, u8),
    MoveSpareToTray(u8, u8),
    MoveTrayToTray(u8, u8, u8),
}

impl From<(&Board, &Board)> for MoveAction {
    fn from(value: (&Board, &Board)) -> Self {
        value.0.neighbors().iter().find(|x| x.1 == *value.1).unwrap().0
    }
}

impl Display for MoveAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use num_ordinal::{Ordinal, Osize};
        match self {
            MoveAction::CollectDragon(d) => write!(f, "Collect {:?} Dragon", d),
            MoveAction::MoveTrayToSpare(source, target) =>
                write!(f, "Move {} Tray's last card to {} Spare ", Osize::from0(*source as usize), Osize::from0(*target as usize)),
            MoveAction::MoveSpareToTray(source, target) =>
                write!(f, "Move {} Spare's card to {} Tray", Osize::from0(*source as usize), Osize::from0(*target as usize)),
            MoveAction::MoveTrayToTray(source, target, num) =>
                write!(f, "Move {} Tray's last {} cards to {} Tray", Osize::from0(*source as usize), num, Osize::from0(*target as usize)),
        }
    }

}

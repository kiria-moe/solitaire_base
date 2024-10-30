use std::fmt::Display;
use crate::card::DragonCard;
use crate::index::Slot;

#[derive(Copy, Clone)]
pub enum MoveAction {
    CollectDragon(DragonCard),
    Move(Slot, usize, Slot),
}

/*impl From<(&Board, &Board)> for MoveAction {
    fn from(value: (&Board, &Board)) -> Self {
        value.0.neighbors().iter().find(|x| x.1 == *value.1).unwrap().0
    }
}*/

impl Display for MoveAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use num_ordinal::{Ordinal, Osize};
        match self {
            MoveAction::CollectDragon(d) => write!(f, "Collect {:?} Dragon", d),
            MoveAction::Move(source, num, target) => match (source, num, target) {
                (Slot::Spare(s), 1, Slot::Tray(t)) =>
                    write!(f, "Move {} Spare's card to {} Tray", Osize::from0(*s as usize), Osize::from0(*t as usize)),
                (Slot::Tray(s), 1, Slot::Spare(t)) =>
                    write!(f, "Move {} Tray's last card to {} Spare", Osize::from0(*s as usize), Osize::from0(*t as usize)),
                (Slot::Tray(s), n, Slot::Tray(t)) =>
                    write!(f, "Move {} Tray's last {} cards to {} Tray", Osize::from0(*s as usize), n, Osize::from0(*t as usize)),
                _ => write!(f, "Invalid Move"),
            }
        }
    }

}

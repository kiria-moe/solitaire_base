#[derive(Copy, Clone)]
pub enum Slot {
    Spare(u8),
    Tray(u8),
}
#[derive(Copy, Clone)]
pub enum Location {
    Spare(u8),
    Tray(u8, u8),
}

pub const ALL_SLOTS: [Slot; 11] = [
    Slot::Spare(0),
    Slot::Spare(1),
    Slot::Spare(2),
    Slot::Tray(0),
    Slot::Tray(1),
    Slot::Tray(2),
    Slot::Tray(3),
    Slot::Tray(4),
    Slot::Tray(5),
    Slot::Tray(6),
    Slot::Tray(7),
];
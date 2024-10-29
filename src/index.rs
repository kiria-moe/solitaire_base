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
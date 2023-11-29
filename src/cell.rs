#[derive(Debug, Clone, Copy)]
pub enum CellStatus {
    Alive,
    Dying { health: u8 },
    Dead,
}

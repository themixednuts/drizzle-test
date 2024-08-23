use crate::columns::SQLiteColumn;

pub trait Autoincrement: Clone + Copy + Default {
    const AUTOINCREMENT: bool;
}

pub trait SQLAutoIncrement: Clone {
    type Value;

    fn autoincrement(self) -> Self::Value;
}

use std::{fmt, marker::PhantomData};

use common::{
    builders::column::ColumnBaseBuilder,
    traits::{Comparable, DefaultFn, DefaultValue, NotNull, PrimaryKey, Unique},
};

use crate::common::Text;

use super::{
    integer::NotAutoIncremented, DefaultFnNotSet, DefaultNotSet, NotPrimary, NotUnique, Nullable,
    SQLiteColumn,
};

#[derive(Debug, Default, Clone, Copy)]
pub enum SQLiteTextMode {
    #[default]
    String,
    Enum,
    JSON,
}

type SQLiteTextColumn<
    TPrimary = NotPrimary,
    TNotNull = Nullable,
    TUnique = NotUnique,
    TDefault = DefaultNotSet,
    TDefaultFn = DefaultFnNotSet,
    Func = fn() -> Result<String, std::fmt::Error>,
> = SQLiteColumn<
    String,
    Text,
    SQLiteTextMode,
    TPrimary,
    TNotNull,
    TUnique,
    NotAutoIncremented,
    TDefault,
    TDefaultFn,
    Func,
>;

pub(crate) fn text(name: &'static str, mode: SQLiteTextMode) -> SQLiteTextColumn {
    SQLiteTextColumn {
        base: ColumnBaseBuilder {
            name,
            mode,
            ..Default::default()
        },
        ..Default::default()
    }
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<String>
    for SQLiteTextColumn<P, N, U, D, F>
{
}
impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<&String>
    for SQLiteTextColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<&str>
    for SQLiteTextColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<Self>
    for SQLiteTextColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<Self>
    for &SQLiteTextColumn<P, N, U, D, F>
{
}

#[cfg(test)]
mod test {
    use super::{text, SQLiteTextMode};
    use core::panic;

    // #[test]
    // fn builder() {
    //     let str = "my text";
    //     let int = text("id", SQLiteTextMode::String)
    //         .primary()
    //         .not_null()
    //         .default(str);

    //     std::thread::spawn(move || {
    //         let int = int;
    //         assert_eq!(int.base.default, Some(str));
    //     });

    //     // .autoincrement()
    //     // .not_null()
    //     // .default(42);
    // }
}

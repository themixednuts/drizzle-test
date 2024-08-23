use common::{
    builders::column::ColumnBaseBuilder,
    traits::{Comparable, DefaultFn, DefaultValue, NotNull, PrimaryKey, Unique},
};

use crate::common::Real;

use super::{
    integer::NotAutoIncremented, DefaultFnNotSet, DefaultNotSet, NotPrimary, NotUnique, Nullable,
    SQLiteColumn,
};

type SQLiteRealColumn<
    TPrimary = NotPrimary,
    TNotNull = Nullable,
    TUnique = NotUnique,
    TDefault = DefaultNotSet,
    TDefaultFn = DefaultFnNotSet,
    TFunc = fn() -> Result<f64, std::fmt::Error>,
> = SQLiteColumn<
    f64,
    Real,
    (),
    TPrimary,
    TNotNull,
    TUnique,
    NotAutoIncremented,
    TDefault,
    TDefaultFn,
    TFunc,
>;

pub(crate) fn real(name: &'static str) -> SQLiteRealColumn {
    SQLiteRealColumn {
        base: ColumnBaseBuilder {
            name,
            ..Default::default()
        },
        ..Default::default()
    }
}
impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<f64>
    for SQLiteRealColumn<P, N, U, D, F>
{
}
impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<&f64>
    for SQLiteRealColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<Self>
    for SQLiteRealColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<Self>
    for &SQLiteRealColumn<P, N, U, D, F>
{
}

#[cfg(test)]
mod test {
    use super::real;
    use core::panic;

    // #[test]
    // fn builder() {
    //     let str = 12.0;
    //     let int = real("id").primary().not_null().default(str);

    //     std::thread::spawn(move || {
    //         let int = int;
    //         assert_eq!(int.base.default, Some(12.0));
    //     });

    // .autoincrement()
    // .not_null()
    // .default(42);
    // }
}

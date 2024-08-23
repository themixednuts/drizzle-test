use common::{
    builders::column::ColumnBaseBuilder,
    traits::{Comparable, DefaultFn, DefaultValue, NotNull, PrimaryKey, Unique},
};

use crate::common::Blob;

use super::{
    integer::NotAutoIncremented, DefaultFnNotSet, DefaultNotSet, NotPrimary, NotUnique, Nullable,
    SQLiteColumn,
};

type SQLiteBlobColumn<
    TPrimary = NotPrimary,
    TNotNull = Nullable,
    TUnique = NotUnique,
    TDefault = DefaultNotSet,
    TDefaultFn = DefaultFnNotSet,
    TFunc = fn() -> Result<Vec<u8>, std::fmt::Error>,
> = SQLiteColumn<
    Vec<u8>,
    Blob,
    (),
    TPrimary,
    TNotNull,
    TUnique,
    NotAutoIncremented,
    TDefault,
    TDefaultFn,
    TFunc,
>;

pub(crate) fn blob(name: &'static str) -> SQLiteBlobColumn {
    SQLiteBlobColumn {
        base: ColumnBaseBuilder {
            name,
            ..Default::default()
        },
        ..Default::default()
    }
}
impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<Vec<u8>>
    for SQLiteBlobColumn<P, N, U, D, F>
{
}
impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<&Vec<u8>>
    for SQLiteBlobColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<&[u8]>
    for SQLiteBlobColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn>
    Comparable<SQLiteBlobColumn<P, N, U, D, F>> for Vec<u8>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<Self>
    for SQLiteBlobColumn<P, N, U, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, D: DefaultValue, F: DefaultFn> Comparable<Self>
    for &SQLiteBlobColumn<P, N, U, D, F>
{
}

#[cfg(test)]
mod test {
    use super::blob;
    use core::panic;

    // #[test]
    // fn builder() {
    //     let b = blob("id").primary().not_null().default(vec![]);

    //     std::thread::spawn(move || {
    //         let b = b;
    //         assert_eq!(b.base.default, Some(vec![]));
    //     });

    //     let b = blob("id").default_fn(|| Ok(vec![]));

    //     // .autoincrement()
    //     // .not_null()
    //     // .default(42);
    // }
}

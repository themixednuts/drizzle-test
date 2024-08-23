use std::marker::PhantomData;

use common::{
    builders::column::ColumnBaseBuilder,
    traits::{ColumnBuilder, Comparable, DefaultFn, DefaultValue, NotNull, PrimaryKey, Unique},
    ToSQL,
};

use crate::{
    common::Integer,
    traits::column::{Autoincrement, SQLAutoIncrement},
};

use super::{
    DefaultFnNotSet, DefaultNotSet, IsPrimary, NotPrimary, NotUnique, Nullable, SQLiteColumn,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct IsAutoIncremented;

impl Autoincrement for IsAutoIncremented {
    const AUTOINCREMENT: bool = true;
}
#[derive(Debug, Default, Clone, Copy)]
pub struct NotAutoIncremented;

impl Autoincrement for NotAutoIncremented {
    const AUTOINCREMENT: bool = false;
}

#[derive(Debug, Default, Clone, Copy)]
pub enum SQLiteIntegerMode {
    #[default]
    Number,
    Timestamp,
    TimestampMS,
    Boolean,
}

type SQLiteIntegerColumn<
    TPrimary = NotPrimary,
    TNotNull = Nullable,
    TUnique = NotUnique,
    TAutoincrement = NotAutoIncremented,
    TDefault = DefaultNotSet,
    TDefaultFn = DefaultFnNotSet,
    TFunc = fn() -> Result<Integer, std::fmt::Error>,
> = SQLiteColumn<
    i64,
    Integer,
    SQLiteIntegerMode,
    TPrimary,
    TNotNull,
    TUnique,
    TAutoincrement,
    TDefault,
    TDefaultFn,
    TFunc,
>;

type SQLiteIntegerColumnBuilderAutoIncrementNotSet<N, U, D, F, Fun> =
    SQLiteIntegerColumn<IsPrimary, N, U, NotAutoIncremented, D, F, Fun>;

type SQLiteIntegerColumnBuilderAutoIncrementSet<N, U, D, F, Fun> =
    SQLiteIntegerColumn<IsPrimary, N, U, IsAutoIncremented, D, F, Fun>;

impl<
        N: NotNull,
        U: Unique,
        D: DefaultValue,
        F: DefaultFn,
        Fun: Fn() -> Result<Integer, std::fmt::Error> + Clone,
    > SQLAutoIncrement for SQLiteIntegerColumnBuilderAutoIncrementNotSet<N, U, D, F, Fun>
{
    type Value = SQLiteIntegerColumnBuilderAutoIncrementSet<N, U, D, F, Fun>;

    fn autoincrement(self) -> Self::Value {
        SQLiteIntegerColumnBuilderAutoIncrementSet {
            base: self.base,
            default: self.default,
            default_fn: self.default_fn,
            unique_name: self.unique_name,
            _marker: PhantomData,
        }
    }
}

impl<
        P: PrimaryKey,
        N: NotNull,
        U: Unique,
        A: Autoincrement,
        D: DefaultValue,
        F: DefaultFn,
        Fun: Fn() -> Result<Integer, std::fmt::Error> + Clone,
    > ToSQL for SQLiteIntegerColumn<P, N, U, A, D, F, Fun>
{
    fn to_sql(self) -> String {
        let name = format!(r#""{}""#, self.base.name);
        let mut sql = vec![name.as_str(), "INTEGER"];

        if P::IS_PRIMARY && !U::IS_UNIQUE {
            sql.push("PRIMARY KEY");
        }

        if A::AUTOINCREMENT {
            sql.push("AUTOINCREMENT");
        }

        if N::IS_NOT_NULL {
            sql.push("NOT NULL");
        }

        sql.join(" ").to_string()
    }
}

pub(crate) fn integer(name: &'static str, mode: SQLiteIntegerMode) -> SQLiteIntegerColumn {
    SQLiteIntegerColumn {
        base: ColumnBaseBuilder {
            name,
            mode,
            ..Default::default()
        },
        ..Default::default()
    }
}

impl<P: PrimaryKey, N: NotNull, U: Unique, A: Autoincrement, D: DefaultValue, F: DefaultFn>
    Comparable<i64> for SQLiteIntegerColumn<P, N, U, A, D, F>
{
}
impl<P: PrimaryKey, N: NotNull, U: Unique, A: Autoincrement, D: DefaultValue, F: DefaultFn>
    Comparable<&i64> for SQLiteIntegerColumn<P, N, U, A, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, A: Autoincrement, D: DefaultValue, F: DefaultFn>
    Comparable<Self> for SQLiteIntegerColumn<P, N, U, A, D, F>
{
}

impl<P: PrimaryKey, N: NotNull, U: Unique, A: Autoincrement, D: DefaultValue, F: DefaultFn>
    Comparable<Self> for &SQLiteIntegerColumn<P, N, U, A, D, F>
{
}

#[cfg(test)]
mod test {

    use common::traits::{SQLDefault, SQLPrimary};

    use crate::traits::column::SQLAutoIncrement;

    use super::{integer, SQLiteIntegerMode};

    #[test]
    fn builder() {
        let num = 42;
        let int = integer("id", SQLiteIntegerMode::Number)
            .default(42)
            .primary()
            .autoincrement();
    }
}

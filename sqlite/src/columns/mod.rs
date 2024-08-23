use std::{
    fmt::{self, Display},
    marker::PhantomData,
};

use common::{
    builders::column::ColumnBaseBuilder,
    traits::{
        ColumnBuilder, DefaultFn, DefaultValue, NotNull, PrimaryKey, SQLDefault, SQLDefaultFn,
        SQLNotNull, SQLPrimary, SQLUnique, Unique,
    },
    ToSQL,
};
use integer::NotAutoIncremented;

use crate::traits::column::Autoincrement;

mod any;
pub mod blob;
pub mod integer;
mod number;
pub mod real;
pub mod text;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct NotSet;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct NoDefaultFn;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct IsPrimary;

impl PrimaryKey for IsPrimary {
    const IS_PRIMARY: bool = true;
}
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct NotPrimary;

impl PrimaryKey for NotPrimary {
    const IS_PRIMARY: bool = false;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct NotNullable;

impl NotNull for NotNullable {
    const IS_NOT_NULL: bool = true;
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct Nullable;

impl NotNull for Nullable {
    const IS_NOT_NULL: bool = false;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct IsUnique;

impl Unique for IsUnique {
    const IS_UNIQUE: bool = true;
}
#[derive(Default, Debug, Clone, Copy)]
pub struct NotUnique;

impl Unique for NotUnique {
    const IS_UNIQUE: bool = false;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DefaultSet;

impl DefaultValue for DefaultSet {
    const HAS_DEFAULT: bool = true;
}
#[derive(Default, Debug, Clone, Copy)]
pub struct DefaultNotSet;

impl DefaultValue for DefaultNotSet {
    const HAS_DEFAULT: bool = false;
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct DefaultFnSet;

impl DefaultFn for DefaultFnSet {
    const HAS_DEFAULT_FN: bool = true;
}
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct DefaultFnNotSet;

impl DefaultFn for DefaultFnNotSet {
    const HAS_DEFAULT_FN: bool = false;
}

#[derive(Debug, Clone)]
pub(crate) struct SQLiteColumn<
    DataType: Default + Clone,
    ColumnType: Default + Clone,
    DataMode: Default + Clone,
    TPrimary: PrimaryKey = NotPrimary,
    TNotNull: NotNull = Nullable,
    TUnique: Unique = NotUnique,
    TAutoincrement: Autoincrement = NotAutoIncremented,
    TDefault: DefaultValue = DefaultNotSet,
    TDefaultFn: DefaultFn = DefaultFnNotSet,
    TFunc: Fn() -> Result<DataType, std::fmt::Error> + Clone = fn() -> Result<
        DataType,
        std::fmt::Error,
    >,
    // TUpdate,
    // TUpdateFn,
    // TUpdateFunc,
    // TConflict,
    // TConflictFn,
    // TConflictFunc,
> {
    pub(crate) base: ColumnBaseBuilder<DataType, ColumnType, DataMode>,
    pub(crate) unique_name: Option<&'static str>,
    pub(crate) default: Option<DataType>,
    pub(crate) default_fn: Option<TFunc>,
    pub(crate) _marker: PhantomData<(
        TPrimary,
        TNotNull,
        TUnique,
        TAutoincrement,
        TDefault,
        TDefaultFn,
    )>,
    // onUpdateFn: Option<Fn2>,
    // uniqueName: Option<String>,
}

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        P: PrimaryKey,
        N: NotNull,
        U: Unique,
        A: Autoincrement,
        D: DefaultValue,
        F: DefaultFn,
    > Default for SQLiteColumn<DataType, ColumnType, DataMode, P, N, U, A, D, F>
{
    fn default() -> Self {
        Self {
            base: ColumnBaseBuilder::default(),
            unique_name: None,
            default: None,
            default_fn: None,
            _marker: PhantomData,
        }
    }
}

// Primary FieldSet
type SQLiteColumnBuilderPrimaryNotSet<DataType, ColumnType, DataMode, N, U, A, D, F, Fun> =
    SQLiteColumn<DataType, ColumnType, DataMode, NotPrimary, N, U, A, D, F, Fun>;

type SQLiteColumnBuilderPrimarySet<DataType, ColumnType, DataMode, N, U, A, D, F, Fun> =
    SQLiteColumn<DataType, ColumnType, DataMode, IsPrimary, N, U, A, D, F, Fun>;

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        N: NotNull,
        U: Unique,
        A: Autoincrement,
        D: DefaultValue,
        F: DefaultFn,
        Fun: Fn() -> Result<DataType, std::fmt::Error> + Sized + Clone,
    > SQLPrimary
    for SQLiteColumnBuilderPrimaryNotSet<DataType, ColumnType, DataMode, N, U, A, D, F, Fun>
{
    type Value = SQLiteColumnBuilderPrimarySet<DataType, ColumnType, DataMode, N, U, A, D, F, Fun>;
    fn primary(self) -> Self::Value {
        SQLiteColumn {
            base: self.base,
            default: self.default,
            default_fn: self.default_fn,
            unique_name: self.unique_name,
            _marker: PhantomData,
        }
    }
}

// Not Null FieldSet
type SQLiteColumnBuilderNotNullNotSet<DataType, ColumnType, DataMode, P, U, A, D, F, Fun> =
    SQLiteColumn<DataType, ColumnType, DataMode, P, Nullable, U, A, D, F, Fun>;

type SQLiteColumnBuilderNotNullSet<DataType, ColumnType, DataMode, P, U, A, D, F, Fun> =
    SQLiteColumn<DataType, ColumnType, DataMode, P, NotNullable, U, A, D, F, Fun>;

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        P: PrimaryKey,
        U: Unique,
        A: Autoincrement,
        D: DefaultValue,
        F: DefaultFn,
        Fun: Fn() -> Result<DataType, std::fmt::Error> + Clone,
    > SQLNotNull
    for SQLiteColumnBuilderNotNullNotSet<DataType, ColumnType, DataMode, P, U, A, D, F, Fun>
{
    type Value = SQLiteColumnBuilderNotNullSet<DataType, ColumnType, DataMode, P, U, A, D, F, Fun>;
    fn not_null(self) -> Self::Value {
        SQLiteColumn {
            base: self.base,
            default: self.default,
            default_fn: self.default_fn,
            unique_name: self.unique_name,
            _marker: PhantomData,
        }
    }
}

// Default FieldSet
type SQLiteColumnBuilderDefaultNotSet<DataType, ColumnType, DataMode, P, N, U, A> =
    SQLiteColumn<DataType, ColumnType, DataMode, P, N, U, A, DefaultNotSet, DefaultFnNotSet>;

type SQLiteColumnBuilderDefaultSet<DataType, ColumnType, DataMode, P, N, U, A> =
    SQLiteColumn<DataType, ColumnType, DataMode, P, N, U, A, DefaultSet, DefaultFnNotSet>;

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        P: PrimaryKey,
        N: NotNull,
        U: Unique,
        A: Autoincrement,
    > SQLDefault for SQLiteColumnBuilderDefaultNotSet<DataType, ColumnType, DataMode, P, N, U, A>
{
    type Value = SQLiteColumnBuilderDefaultSet<DataType, ColumnType, DataMode, P, N, U, A>;
    type DataType = DataType;

    fn default(self, value: Self::DataType) -> Self::Value {
        SQLiteColumn {
            base: self.base,
            default: Some(value),
            default_fn: self.default_fn,
            unique_name: self.unique_name,
            _marker: PhantomData,
        }
    }
}

// DefaultFn FieldSet
type SQLiteColumnBuilderDefaultFnNotSet<DataType, ColumnType, DataMode, P, N, U, A> =
    SQLiteColumn<DataType, ColumnType, DataMode, P, N, U, A, DefaultNotSet, DefaultFnNotSet>;

type SQLiteColumnBuilderDefaultFnSet<DataType, ColumnType, DataMode, P, N, U, A, F> =
    SQLiteColumn<DataType, ColumnType, DataMode, P, N, U, A, DefaultNotSet, DefaultFnSet, F>;

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        P: PrimaryKey,
        N: NotNull,
        U: Unique,
        A: Autoincrement,
    > SQLDefaultFn
    for SQLiteColumnBuilderDefaultFnNotSet<DataType, ColumnType, DataMode, P, N, U, A>
{
    type DataType = DataType;
    type Error = fmt::Error;
    type Value<Func: Fn() -> Result<DataType, fmt::Error> + Clone> =
        SQLiteColumnBuilderDefaultFnSet<DataType, ColumnType, DataMode, P, N, U, A, Func>;

    fn default_fn<Func>(self, value: Func) -> Self::Value<Func>
    where
        Func: Fn() -> Result<DataType, fmt::Error> + Clone,
    {
        SQLiteColumn {
            base: self.base,
            default: self.default,
            default_fn: Some(value),
            unique_name: self.unique_name,
            _marker: PhantomData,
        }
    }
}

// Unique FieldSet
type SQLiteColumnBuilderUniqueNotSet<DataType, ColumnType, DataMode, N, A, D, F, Fun> =
    SQLiteColumn<DataType, ColumnType, DataMode, NotPrimary, N, NotUnique, A, D, F, Fun>;

type SQLiteColumnBuilderUniqueSet<DataType, ColumnType, DataMode, N, A, D, F, Fun> =
    SQLiteColumn<DataType, ColumnType, DataMode, NotPrimary, N, IsUnique, A, D, F, Fun>;

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        N: NotNull,
        A: Autoincrement,
        D: DefaultValue,
        F: DefaultFn,
        Fun: Fn() -> Result<DataType, std::fmt::Error> + Clone,
    > SQLUnique
    for SQLiteColumnBuilderUniqueNotSet<DataType, ColumnType, DataMode, N, A, D, F, Fun>
{
    type Value = SQLiteColumnBuilderUniqueSet<DataType, ColumnType, DataMode, N, A, D, F, Fun>;

    fn unique(self, value: &'static str) -> Self::Value {
        SQLiteColumn {
            base: self.base,
            default: self.default,
            default_fn: self.default_fn,
            unique_name: Some(value),
            _marker: PhantomData,
        }
    }
}

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        TPrimary: PrimaryKey,
        TNotNull: NotNull,
        TUnique: Unique,
        TAutoincrement: Autoincrement,
        TDefault: DefaultValue,
        TDefaultFn: DefaultFn,
        TFunc: Fn() -> Result<DataType, std::fmt::Error> + Clone,
    > Display
    for SQLiteColumn<
        DataType,
        ColumnType,
        DataMode,
        TPrimary,
        TNotNull,
        TUnique,
        TAutoincrement,
        TDefault,
        TDefaultFn,
        TFunc,
    >
where
    Self: ToSQL,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone().to_sql())
    }
}

impl<
        DataType: Default + Clone,
        ColumnType: Default + Clone,
        DataMode: Default + Clone,
        TPrimary: PrimaryKey,
        TNotNull: NotNull,
        TUnique: Unique,
        TAutoincrement: Autoincrement,
        TDefault: DefaultValue,
        TDefaultFn: DefaultFn,
        TFunc: Fn() -> Result<DataType, std::fmt::Error> + Clone,
    > ColumnBuilder
    for SQLiteColumn<
        DataType,
        ColumnType,
        DataMode,
        TPrimary,
        TNotNull,
        TUnique,
        TAutoincrement,
        TDefault,
        TDefaultFn,
        TFunc,
    >
{
    fn name(&self) -> &str {
        self.base.name
    }

    fn build(self) -> Self {
        self
    }
}

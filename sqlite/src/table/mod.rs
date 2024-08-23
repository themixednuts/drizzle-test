use std::marker::PhantomData;

use crate::traits::table::Strict;

#[derive(Debug, Default)]
struct NotStrict {}

impl Strict for NotStrict {
    const IS_STRICT: bool = false;
}

#[derive(Debug, Default)]
struct IsStrict {}

impl Strict for IsStrict {
    const IS_STRICT: bool = true;
}

#[derive(Debug, Default)]
struct WithoutRowID {}

#[derive(Debug, Default)]
struct SQLiteTableBuilder<S = NotStrict> {
    name: &'static str,
    _strict: PhantomData<S>,
}

mod columns;
mod common;
mod table;
mod traits;

// Main macro to construct the SQLiteColumn type with the appropriate generics
macro_rules! sqlite_column_type {
    (integer $(.$func:ident)*) => {
        SQLiteIntegerColumnBuilder<
            detect_primary_key!($(.$func)*),
            detect_not_null!($(.$func)*),
            detect_unique!($(.$func)*),
            detect_autoincrement!($(.$func)*),
            detect_default!($(.$func)*),
            detect_default_fn!($(.$func)*),
        >
    };
    (real $(.$func:ident)*) => {
        SQLiteRealColumnBuilder<
            detect_primary_key!($(.$func)*),
            detect_not_null!($(.$func)*),
            detect_unique!($(.$func)*),
            detect_default!($(.$func)*),
            detect_default_fn!($(.$func)*),
        >
    };
    (text $(.$func:ident)*) => {
        SQLiteTextColumnBuilder<
            detect_primary_key!($(.$func)*),
            detect_not_null!($(.$func)*),
            detect_unique!($(.$func)*),
            detect_default!($(.$func)*),
            detect_default_fn!($(.$func)*),
        >
    };
    (blob $(.$func:ident)*) => {
        SQLiteBlobColumnBuilder<
            detect_primary_key!($(.$func)*),
            detect_not_null!($(.$func)*),
            detect_unique!($(.$func)*),
            detect_default!($(.$func)*),
            detect_default_fn!($(.$func)*),
        >
    };
}

// Detect specific function and assign corresponding type for PrimaryKey
macro_rules! detect_primary_key {
    (.primary $(.$func:ident)*) => { IsPrimary };
    (.$head:ident $(.$func:ident)*) => {
        detect_primary_key!($(.$func)*)
    };
    () => { NotPrimary };
}

// Detect specific function and assign corresponding type for NotNull
macro_rules! detect_not_null {
    ($(.)?not_null $(.$func:ident)*) => {
        NotNullable
    };
    ($(.)?$head:ident $(.$func:ident)*) => {
        detect_not_null!($(.$func)*)
    };
    ($(.)?) => {
        Nullable
    };
}

// Detect specific function and assign corresponding type for Unique
macro_rules! detect_unique {
    (.unique $(.$func:ident)*) => {
        IsUnique
    };
    (.$head:ident $(.$func:ident)*) => {
        detect_unique!($(.$func)*)
    };
    ($(.)?) => {
        NotUnique
    };
}

// Detect specific function and assign corresponding type for Autoincremented
macro_rules! detect_autoincrement {
    (.autoincrement $(.$func:ident)*) => {
        IsAutoIncremented
    };
    (.$head:ident $(.$func:ident)*) => {
        detect_autoincrement!($(.$func)*)
    };
    () => {
        NotAutoIncremented
    };
}

// Detect specific function and assign corresponding type for Default
macro_rules! detect_default {
    (.default $(.$func:ident)*) => {
        DefaultSet
    };
    (.$head:ident $(.$func:ident)*) => {
        detect_default!($(.$func)*)
    };
    () => {
        DefaultNotSet
    };
}

// Detect specific function and assign corresponding type for DefaultFn
macro_rules! detect_default_fn {
    ($(.)?default_fn $(.$func:ident)*) => {
        $crate::DefaultFnSet
    };
    ($(.)?$head:ident $(.$func:ident)*) => {
        detect_default_fn!($(.$func)*)
    };
    () => {
        DefaultFnNotSet
    };
}

macro_rules! sqlite_table {
    ($table_name:expr, { $($field_name:ident : $func:tt),* }) => {{
            let $( $field_name = $func; )*
    }};
    ($table_name:expr, { $($field_name:ident : $type:ident ( $($type_args:tt)* ) $(.$func:ident ( $($args:expr)* ))*),* $(,)? }) => {
        paste::paste! {

            // stringify!($( $field_name: sqlite_column_type!($type $(.$func)*), )*);
            // stringify!($($type($($type_args)*)$(.$func($($args),*))*),*);
            // struct [<$table_name:camel>] {
                // $( $field_name: sqlite_column_type!($type $(.$func)*), )*
            // }

            $( let $field_name = $type($($type_args)*)$(.$func($($args),*))*; )*

            // impl [<$table_name:camel>] {
            //     pub fn new() -> Self {
            //         [<$table_name:camel>] {
            //             $( $field_name: $type($($type_args)*)$(.$func($($args),*))*, )*
            //         }
            //     }
            // }
        }
    };
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, fmt::Display};

    use ::common::traits::{
        ColumnBuilder, Comparable, DefaultFn, DefaultValue, NotNull, PrimaryKey, Unique,
    };
    use columns::{
        integer::{self, integer, SQLiteIntegerMode},
        real::real,
        text::{text, SQLiteTextMode},
        SQLiteColumn,
    };
    use traits::column::*;

    use super::*;

    #[test]
    fn table() {
        fn eq<Lhs, Rhs>(lhs: &Lhs, rhs: &Rhs)
        where
            Lhs: Comparable<Rhs>,
        {
        }

        struct SQLiteTable {
            id: Box<SQLiteColumn<DT, CT, DM, P, N, U, A, D, F, Func>>,
        }

        impl SQLiteTable {
            fn column<
                DataType: Default + Clone,
                ColumnType: Default + Clone,
                DataMode: Default + Clone,
                TPrimary: PrimaryKey,
                TNotNull: NotNull,
                TUnique: Unique,
                TAuto: Autoincrement,
                TDefault: DefaultValue,
                TDefaultFn: DefaultFn,
            >(
                &mut self,
                column: SQLiteColumn<
                    DataType,
                    ColumnType,
                    DataMode,
                    TPrimary,
                    TNotNull,
                    TUnique,
                    TAuto,
                    TDefault,
                    TDefaultFn,
                >,
            ) -> &mut Self {
                self
            }

            // fn columns(&mut self, columns: Iterator<Item = dyn ColumnBuilder>) {}
        }

        let table = SQLiteTable {}
            .column(integer("id", SQLiteIntegerMode::Number))
            .column(real("value"));

        macro_rules! sql {
            // Capture the format string and a list of expressions
            ($($arg:tt)*) => {{
                // Use format_args! to generate a formatted string
                let query = format!($($arg)*);
                // SQL { query }
                query
            }};
        }

        // let sql = sql!("SELECT * FROM users WHERE id = {}", table.id);

        fn my_fn() -> Result<i64, std::fmt::Error> {
            Ok(42)
        }

        // sqlite_table!("users_table", {
        //     id: integer("id", SQLiteIntegerMode::Number).primary().not_null().default(42),
        //     // name: text("name", SQLiteTextMode::String).primary().not_null(),
        // });

        // let users = &UsersTable::new();
        // let id = &users.id;
        // let name = &users.name;

        // let f = 42;

        // let n = users.id.name();

        // println!("NAME NAME NAME NAME NAME: {n}");
    }
}

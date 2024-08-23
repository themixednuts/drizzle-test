pub trait PrimaryKey: Clone + Copy + Default {
    const IS_PRIMARY: bool;
}

pub trait NotNull: Clone + Copy + Default {
    const IS_NOT_NULL: bool;
}

pub trait Unique: Clone + Copy + Default {
    const IS_UNIQUE: bool;
}

pub trait DefaultValue: Clone + Copy + Default {
    const HAS_DEFAULT: bool;
}

pub trait DefaultFn: Clone + Copy + Default {
    const HAS_DEFAULT_FN: bool;
}

pub trait SQLPrimary: Clone {
    type Value;
    fn primary(self) -> Self::Value;
}

pub trait SQLNotNull: Clone {
    type Value;
    fn not_null(self) -> Self::Value;
}

pub trait SQLUnique: Clone {
    type Value;

    fn unique(self, name: &'static str) -> Self::Value;
}

pub trait SQLDefault: Clone {
    type DataType;
    type Value;

    fn default(self, value: Self::DataType) -> Self::Value;
}

pub trait SQLDefaultFn: Clone {
    type DataType;
    type Error;
    type Value<F: Fn() -> Result<Self::DataType, Self::Error> + Clone>;

    fn default_fn<F>(self, fun: F) -> Self::Value<F>
    where
        F: (Fn() -> Result<Self::DataType, Self::Error>) + Clone;
}

pub trait ColumnBuilder {
    fn name(&self) -> &str;
    fn build(self) -> Self;
}

mod primitives;
#[macro_use]
mod collections;
mod array;
mod containers;
mod string;
mod tuple;

pub trait Dummy<T> {
    /// take reference to config `T` and generate Self
    fn dummy_ref(config: &T) -> Self;

    /// consume config `T` and generate Self    
    fn dummy(config: T) -> Self
    where
        Self: Sized,
    {
        Self::dummy_ref(&config)
    }
}

pub trait DummyAny: Dummy<crate::any::Any> {
    fn any() -> Self;
}

impl<T: Dummy<crate::any::Any>> DummyAny for T {
    fn any() -> Self {
        Self::dummy(ANY)
    }
}

pub mod any {
    pub struct Any;
    pub const ANY: Any = Any;
}
pub use crate::any::ANY;

pub mod distributions {
    /// re-exports from `rand` crate
    pub use rand::distributions::Uniform;
}

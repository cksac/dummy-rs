#[macro_use]
mod vec;
mod primitives;

pub trait Dummy<T> {
    /// take reference to config `T` and genreate Self
    fn dummy_ref(config: &T) -> Self;

    /// consume config `T` and genreate Self    
    fn dummy(config: T) -> Self
    where
        Self: Sized,
    {
        Self::dummy_ref(&config)
    }
}

pub mod any {
    pub struct Any;
    pub const ANY: Any = Any;
}
pub use self::any::ANY;

pub mod distributions {
    // re-exports
    pub use rand::distributions::Uniform;
}

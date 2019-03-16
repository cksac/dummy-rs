use crate::any::Any;
use crate::{Dummy, DummyAny};

impl<T, E> Dummy<Any> for Result<T, E>
where
    T: Dummy<Any>,
    E: Dummy<Any>,
{
    fn dummy_ref(_: &Any) -> Self {
        if bool::any() {
            Ok(T::any())
        } else {
            Err(E::any())
        }
    }
}

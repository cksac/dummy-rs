use crate::any::Any;
use crate::{Dummy, DummyAny};

impl<T> Dummy<Any> for Option<T>
where
    T: Dummy<Any>,
{
    fn dummy_ref(_: &Any) -> Self {
        if bool::any() {
            Some(T::any())
        } else {
            None
        }
    }
}

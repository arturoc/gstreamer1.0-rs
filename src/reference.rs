use std::ops::{Deref, DerefMut};

pub trait Reference{
    fn reference(&self) -> Self where Self:Sized;
}

/// Represents a reference to a gstreamer object
/// equivalent to gst_object_ref and gst_mini_object_ref
/// it auto unrefs when going out of scope
pub struct Ref<T>{
    value: T
}

impl<T:Reference> Clone for Ref<T>{
    fn clone(&self) -> Ref<T>{
        Ref{ value: self.value.reference() }
    }
}

impl<T:Reference> Ref<T>{
    pub fn new(t: &T) -> Ref<T>{
        Ref{ value: t.reference() }
    }
}

impl<T> Deref for Ref<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.value
    }
}

impl<T> DerefMut for Ref<T>{
    fn deref_mut(&mut self) -> &mut T{
        &mut self.value
    }
}

impl<T:Reference> From<T> for Ref<T>{
    fn from(t: T) -> Ref<T>{
        Ref{ value: t }
    }
}

impl<T> AsRef<T> for Ref<T>{
    fn as_ref(&self) -> &T{
        &self.value
    }
}

impl<T> AsMut<T> for Ref<T>{
    fn as_mut(&mut self) -> &mut T{
        &mut self.value
    }
}

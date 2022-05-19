use super::assets::HandleId;
use std::marker::PhantomData;

pub struct Handle<T>{
    pub id: HandleId,
    pub(crate) marker: PhantomData<T>,
}

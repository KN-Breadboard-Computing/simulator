use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use slotmap::new_key_type;

pub use slotmap::{Key, KeyData};

new_key_type! {
    pub struct ComponentId;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TypedId<C> {
    inner: ComponentId,
    marker: PhantomData<C>,
}

impl<C> From<TypedId<C>> for ComponentId {
    fn from(value: TypedId<C>) -> Self {
        value.inner
    }
}

impl<C> From<ComponentId> for TypedId<C> {
    fn from(value: ComponentId) -> Self {
        Self { inner: value, marker: PhantomData }
    }
}
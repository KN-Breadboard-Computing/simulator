use std::marker::PhantomData;

use slotmap::new_key_type;

new_key_type! {pub struct NodeId;}

#[derive(Debug, Clone, Copy)]
pub struct TypedId<C> {
    inner: NodeId,
    marker: PhantomData<C>,
}

impl<C> From<TypedId<C>> for NodeId {
    fn from(value: TypedId<C>) -> Self {
        value.inner
    }
}

impl<C> From<NodeId> for TypedId<C> {
    fn from(value: NodeId) -> Self {
        Self { inner: value, marker: PhantomData }
    }
}
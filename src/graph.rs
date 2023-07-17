use std::{collections::VecDeque, marker::PhantomData, ops::Deref};

use bitvec::prelude::*;
use slotmap::{SecondaryMap, SlotMap};

use crate::components::Component;

use self::node::{Node, InnerNodeRef, Slot};
pub mod node;

#[derive(Debug)]
pub struct Graph {
    nodes: SlotMap<InnerNodeRef, Node>,
    inputs: SecondaryMap<InnerNodeRef, BitVec>,
    outputs: SecondaryMap<InnerNodeRef, BitVec>,
}

#[derive(Debug, Clone, Copy)]
pub struct CompRef<C> {
    inner: InnerNodeRef,
    marker: PhantomData<C>
}

impl<C> From<CompRef<C>> for InnerNodeRef {
    fn from(value: CompRef<C>) -> Self {
        value.inner
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            inputs: SecondaryMap::new(),
            outputs: SecondaryMap::new(),
        }
    }

    pub fn add_component<C: Component + 'static>(&mut self, component: C) -> CompRef<C> {
        let input_size = C::input_size();
        let output_size = C::output_size();

        let node = Node {
            component: Box::new(component),
            input_slots: vec![None; input_size],
            output_slots: vec![None; output_size],
        };

        let node_ref = self.nodes.insert(node);

        self.inputs.insert(node_ref, bitvec![0;input_size]);
        self.outputs.insert(node_ref, bitvec![0;output_size]);

        CompRef { inner: node_ref, marker: PhantomData }
    }

    pub fn add_connection(
        &mut self,
        node_a: impl Into<InnerNodeRef>,
        slot_a: usize,
        node_b: impl Into<InnerNodeRef>,
        slot_b: usize,
    ) {
        let node_a = node_a.into();
        let node_b = node_b.into();

        self.nodes[node_a].output_slots[slot_a] = Some(Slot {
            target_node: node_b,
            target_slot: slot_b,
        });
        self.nodes[node_b].input_slots[slot_b] = Some(Slot {
            target_node: node_a,
            target_slot: slot_a,
        });
    }

    pub fn propagate_from(&mut self, node: impl Into<InnerNodeRef>) {
        let node = node.into();

        let mut queue = VecDeque::new();
        queue.push_back(node);

        let mut in_queue = SecondaryMap::new();
        in_queue.insert(node, ());

        while let Some(next_node_ref) = queue.pop_front() {
            in_queue.remove(next_node_ref);

            let next_node = &mut self.nodes[next_node_ref];
            let input = &self.inputs[next_node_ref];
            let output = &mut self.outputs[next_node_ref];

            next_node.component.propagate(input, output);

            for (i, out_slot) in next_node.output_slots.iter().enumerate() {
                let Some(out_slot) = out_slot else {continue;};
                let output_bit = output[i];
                self.inputs[out_slot.target_node].set(out_slot.target_slot, output_bit);

                if !in_queue.contains_key(out_slot.target_node) {
                    in_queue.insert(out_slot.target_node, ());
                    queue.push_back(out_slot.target_node);
                }
            }
        }
    }

    pub fn get_component<C : 'static>(&self, c_ref: CompRef<C>) -> &C {
        self.nodes[c_ref.into()].component.as_any().downcast_ref().unwrap()
    }

    pub fn get_component_mut<C : 'static>(&mut self, c_ref: CompRef<C>) -> &mut C {
        self.nodes[c_ref.into()].component.as_any_mut().downcast_mut().unwrap()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

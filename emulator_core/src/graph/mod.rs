use std::{collections::VecDeque, ops::{Index, IndexMut}};

use bitvec::prelude::*;
use slotmap::{SecondaryMap, SlotMap};

use crate::components::{Component, ComponentBehaviour};
use self::{node::{Node, Slot}, id::{NodeId, TypedId}};

pub mod node;
pub mod id;

#[derive(Debug)]
pub struct Graph {
    pub nodes: SlotMap<NodeId, Node>,
    pub inputs: SecondaryMap<NodeId, BitVec>,
    pub outputs: SecondaryMap<NodeId, BitVec>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            inputs: SecondaryMap::new(),
            outputs: SecondaryMap::new(),
        }
    }

    pub fn add_comp<C: Into<Component>>(&mut self, component: C) -> TypedId<C> {
        let component = component.into();
        let input_size = component.input_size();
        let output_size = component.output_size();

        let node = Node {
            component,
            input_slots: vec![None; input_size],
            output_slots: vec![None; output_size],
        };

        let node_ref = self.nodes.insert(node);

        self.inputs.insert(node_ref, bitvec![0;input_size]);
        self.outputs.insert(node_ref, bitvec![0;output_size]);

        node_ref.into()
    }

    pub fn remove_comp(&mut self, node: impl Into<NodeId>) {
        let removed = self.nodes.remove(node.into()).unwrap();

        for input in removed.input_slots {
            let Some(input) = input else {continue;};
            self.nodes[input.target_node].output_slots[input.target_slot] = None;
        }

        for output in removed.output_slots {
            let Some(output) = output else {continue;};
            self.nodes[output.target_node].input_slots[output.target_slot] = None;
        }
    }

    pub fn add_conn(
        &mut self,
        node_a: impl Into<NodeId>,
        slot_a: usize,
        node_b: impl Into<NodeId>,
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

    pub fn remove_conn(
        &mut self,
        node_a: impl Into<NodeId>,
        slot_a: usize,
        node_b: impl Into<NodeId>,
        slot_b: usize,
    ) {
        let node_a = node_a.into();
        let node_b = node_b.into();

        self.nodes[node_a].output_slots[slot_a] = None;
        self.nodes[node_b].input_slots[slot_b] = None;
    }

    pub fn propagate_from(&mut self, node: impl Into<NodeId>) {
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

    pub fn add_input_slot(&mut self, node: impl Into<NodeId>) {
        let node = node.into();
        self.nodes[node].input_slots.push(None);
        self.inputs[node].push(false);
    }

    pub fn add_output_slot(&mut self, node: impl Into<NodeId>) {
        let node = node.into();
        self.nodes[node].output_slots.push(None);
        self.outputs[node].push(false);
    }
}

impl Index<NodeId> for Graph {
    type Output = Component;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index].component
    }
}

impl IndexMut<NodeId> for Graph {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index].component
    }
}

impl<C> Index<TypedId<C>> for Graph where Component : AsRef<C> {
    type Output = C;

    fn index(&self, index: TypedId<C>) -> &Self::Output {
        self.nodes[index.into()].component.as_ref()
    }
}

impl<C> IndexMut<TypedId<C>> for Graph where Component : AsMut<C>  + AsRef<C> {
    fn index_mut(&mut self, index: TypedId<C>) -> &mut Self::Output {
        self.nodes[index.into()].component.as_mut()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

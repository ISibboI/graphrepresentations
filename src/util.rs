use crate::{EdgeId, NodeId};
use std::ops::AddAssign;

pub trait PrefixSum<T> {
    fn prefix_sum(&mut self) -> T;
}

impl<T: AddAssign + Default + Clone> PrefixSum<T> for Vec<T> {
    fn prefix_sum(&mut self) -> T {
        let mut accumulator = T::default();
        for item in self.iter_mut() {
            let value = item.clone();
            *item += accumulator.clone();
            accumulator += value;
        }
        accumulator
    }
}

impl PrefixSum<NodeId> for Vec<NodeId> {
    fn prefix_sum(&mut self) -> NodeId {
        let mut accumulator = 0;
        for item in self.iter_mut() {
            let value = item.id;
            item.id += accumulator;
            accumulator += value;
        }
        NodeId::new(accumulator)
    }
}

impl PrefixSum<EdgeId> for Vec<EdgeId> {
    fn prefix_sum(&mut self) -> EdgeId {
        let mut accumulator = 0;
        for item in self.iter_mut() {
            let value = item.id;
            item.id += accumulator;
            accumulator += value;
        }
        EdgeId::new(accumulator)
    }
}

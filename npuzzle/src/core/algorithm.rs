use std::ops::{Deref, DerefMut};

use super::{Order, Problem, node::Node};

pub trait Algorithm: Deref<Target = AlgorithmCommon> + DerefMut {
    fn new() -> Self
    where
        Self: Sized,
        Self: Default,
    {
        Self::default()
    }
    fn run(&mut self, state: Problem, order: &Order) -> Option<Node>;

    fn get_reached_depth(&self) -> i16 {
        self.reached_depth
    }

    fn get_visited_count(&self) -> usize {
        self.visited_count
    }

    fn get_processed_count(&self) -> usize {
        self.processed_count
    }

    fn get_result_len(&self) -> u8 {
        self.result_len
    }

    fn set_processed_count(&mut self, count: usize) {
        self.processed_count = count;
    }

    fn set_visited_count(&mut self, count: usize) {
        self.visited_count = count;
    }

    fn set_reached_depth(&mut self, depth: i16) {
        self.reached_depth = depth;
    }

    fn set_result_len(&mut self, len: u8) {
        self.result_len = len;
    }
}

#[derive(Default)]
pub struct AlgorithmCommon {
    pub visited_count: usize,
    pub processed_count: usize,
    pub reached_depth: i16,
    pub result_len: u8,
}

impl AlgorithmCommon {
    pub fn new() -> Self {
        Self::default()
    }
}

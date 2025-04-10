use std::ops::Add;

use super::Dir;

#[derive(Debug)]
pub struct Solution {
    pub visited_count: usize,
    pub processed_count: usize,
    pub reached_depth: i16,
    pub result_len: u8,
    pub duration: f64,
    pub path: Vec<Dir>,
}
impl Clone for Solution {
    fn clone(&self) -> Solution {
        Self {
            path: vec![],
            ..*self
        }
    }
}

impl Add<Solution> for Solution {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            visited_count: self.visited_count + other.visited_count,
            processed_count: self.processed_count + other.processed_count,
            reached_depth: self.reached_depth + other.reached_depth,
            result_len: self.result_len + other.result_len,
            duration: self.duration + other.duration,
            path: vec![],
        }
    }
}

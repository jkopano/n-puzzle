use std::time::Instant;

use crate::algorithms::{AStar, Bfs, Dfs};

use super::{Algorithm, Dir, Order, Problem, Solution, node::Node};

pub enum SolverEnum {
    Bfs(BfsSolver),
    Dfs(DfsSolver),
    AStar(AStarSolver),
}
impl Clone for SolverEnum {
    fn clone(&self) -> Self {
        match self {
            SolverEnum::Bfs(solver) => SolverEnum::Bfs((*solver).clone()),
            SolverEnum::Dfs(solver) => SolverEnum::Dfs((*solver).clone()),
            SolverEnum::AStar(solver) => SolverEnum::AStar((*solver).clone()),
        }
    }
}

pub type BfsSolver = Solver<Bfs>;
pub type DfsSolver = Solver<Dfs>;
pub type AStarSolver = Solver<AStar>;

pub struct Solver<T: Algorithm> {
    pub order: Order,
    pub algorithm: T,
}

impl<T: Algorithm + Default> Clone for Solver<T> {
    fn clone(&self) -> Self {
        Self {
            order: self.order.clone(),
            algorithm: T::new(),
        }
    }
}

impl<T: Algorithm + Default> Solver<T> {
    pub fn new(order: Order) -> Self {
        Self {
            algorithm: T::new(),
            order,
        }
    }

    pub fn solve(&mut self, problem: Problem) -> Option<Solution> {
        let start = Instant::now();
        let result = self.algorithm.run(problem, &self.order);
        let duration = start.elapsed().as_micros() as f64 / 1000.0;

        if let Some(result) = result {
            return Some(Solution {
                visited_count: self.algorithm.get_visited_count(),
                processed_count: self.algorithm.get_processed_count(),
                reached_depth: self.algorithm.get_reached_depth(),
                result_len: self.algorithm.get_result_len(),
                duration,
                path: Solver::<T>::make_path(result),
            });
        }

        None
    }

    fn make_path(state: Node) -> Vec<Dir> {
        let mut directions = Vec::new();

        for x in state.dir_iter() {
            directions.push(x);
        }

        directions.reverse();
        directions
    }
}

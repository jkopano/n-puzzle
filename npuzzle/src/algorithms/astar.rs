use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use algorithm_derive::Algorithm;

use crate::core::{Algorithm, AlgorithmCommon, Dir, Order, Problem, node::Node};

#[derive(Default, Algorithm)]
pub struct AStar {
    common: AlgorithmCommon,
}

impl Algorithm for AStar {
    /// Executes the A* search algorithm to solve the given puzzle problem.
    ///
    /// # Arguments
    /// * `problem` - The initial puzzle state to solve
    /// * `order` - The move ordering and heuristic to use (e.g., Manhattan distance)
    ///
    /// # Returns
    /// * `Some(Node)` containing the solved state if a solution is found
    /// * `None` if no solution exists or if the search space is exhausted
    ///
    /// # Behavior
    /// 1.Initializes priority queue (open set) and visited states hash map
    /// 2.Uses the specified heuristic function to guide the search
    /// 3.Expands nodes in order of (heuristic + path cost)
    /// 4.Tracks visited states to avoid cycles
    /// 5.Updates algorithm statistics (visited count, depth reached, etc.)
    ///
    /// # Performance Characteristics
    /// * Time complexity: O(b^d) where b is branching factor, d is solution depth
    /// * Space complexity: O(b^d) for storing visited states
    /// * Complete: Will find solution if one exists
    /// * Optimal: Finds shortest path when using admissible heuristic
    ///
    fn run(&mut self, problem: Problem, order: &Order) -> Option<Node> {
        let mut open = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut dirs = Dir::values();
        dirs.reverse();

        let heuristic_fn = order.get_heuristic();

        let state = Node::new(problem.get());
        visited.insert(state.hash_code(), 0);
        let mut count = 0;

        open.push(Reverse((heuristic_fn(state.get_board(), 0), state)));

        while let Some(Reverse((_, current))) = open.pop() {
            count += 1;

            if current.depth() > *visited.get(&current.hash_code()).unwrap_or(&u8::MAX) {
                continue;
            }

            for child in current.get_child_nodes(dirs) {
                if child.is_solved() {
                    self.set_visited_count(visited.len());
                    self.set_reached_depth(child.depth() as i16);
                    self.set_result_len(child.depth());
                    self.set_processed_count(count);
                    return Some(child.clone());
                }

                let depth = child.depth();
                let child_hash = child.hash_code();

                if depth < *visited.get(&child_hash).unwrap_or(&u8::MAX) {
                    visited.insert(child_hash, depth);

                    open.push(Reverse((heuristic_fn(child.get_board(), depth), child)));
                }
            }
        }

        self.set_reached_depth(-1);
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{Algorithm, Order, Problem};

    use super::AStar;
    // Helper function to set up a solved state
    fn solved_state() -> Problem {
        Problem::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12], // Correct final state
            vec![13, 14, 15, 0], // Correct final state
        ])
    }

    // Helper function to set up an unsolved state
    fn unsolved_state() -> Problem {
        Problem::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 0],
            vec![13, 14, 15, 12], // Correct final state
        ])
    }

    // Test if BFS finds a solved state immediately
    #[test]
    fn test_astar_immediate_solution() {
        let solved_state = solved_state();
        let mut astar = AStar::new();

        // Since the initial state is solved, Astar should return the solved state immediately
        let result = astar.run(solved_state.clone(), &Order::Manh);
        assert!(result.is_some());
        assert_eq!(
            *result.unwrap().get_board(),
            solved_state
                .get()
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<u8>>()
        );
    }

    // Test if Astar can solve a puzzle
    #[test]
    fn test_astar_solving_puzzle() {
        let unsolved_state = unsolved_state();
        let mut astar = AStar::new();

        // Astar should be able to find the solved state
        let result = astar.run(unsolved_state.clone(), &Order::Manh);
        assert!(result.is_some());
        assert_eq!(
            *result.unwrap().get_board(),
            solved_state()
                .get()
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<u8>>()
        );
    }
    //
    // Test Astar on a very small puzzle (e.g., 2x2)
    #[test]
    fn test_astar_small_puzzle() {
        let small_unsolved_board = Problem::new(vec![
            vec![1, 0],
            vec![3, 2], // Misplaced pieces
        ]);
        let small_solved_board = vec![
            vec![1, 2],
            vec![3, 0], // Correct final state
        ];
        let unsolved_state = small_unsolved_board;
        let solved_state = small_solved_board;
        let mut astar = AStar::new();

        // Astar should return the solved state for the 2x2 puzzle
        let result = astar.run(unsolved_state.clone(), &Order::Manh);
        assert!(result.is_some());
        assert_eq!(
            *result.unwrap().get_board(),
            solved_state.iter().flatten().copied().collect::<Vec<u8>>()
        );
    }

    // Test Astar on an empty board (should panic based on your implementation)
    #[test]
    #[should_panic]
    fn test_astar_empty_board() {
        let empty_board: Vec<Vec<u8>> = Vec::new();
        let mut astar = AStar::new(); // This should panic because the state is empty
        astar.run(Problem::new(empty_board), &Order::Manh);
    }
}

use std::collections::HashMap;

use algorithm_derive::Algorithm;

use crate::core::{Algorithm, Order, Problem, algorithm::AlgorithmCommon, node::Node};

use super::MAX_DEPTH;

#[derive(Default, Algorithm)]
pub struct Dfs {
    common: AlgorithmCommon,
}

impl Algorithm for Dfs {
    /// Executes a depth-first search (DFS) algorithm to solve the puzzle.
    ///
    /// # Arguments
    /// * `state` - The initial puzzle state to solve
    /// * `order` - The move ordering to use (must be `Order::Perm` variant)
    ///
    /// # Returns
    /// * `Some(Node)` containing the solved state if found within `MAX_DEPTH`
    /// * `None` if no solution exists or maximum depth is reached
    ///
    /// # Behavior
    /// 1. Uses a stack for LIFO processing (depth-first exploration)
    /// 2. Expands nodes in reverse order of specified directions (for DFS)
    /// 3. Tracks visited states with their depth to prune worse paths
    /// 4. Updates algorithm statistics upon solution discovery
    /// 5. Enforces `MAX_DEPTH` limit to prevent infinite recursion
    ///
    /// # Panics
    /// Will panic if `order` is not the `Order::Perm` variant
    ///
    /// # Performance Characteristics
    /// * Time complexity: O(b^m) where b is branching factor, m is maximum depth
    /// * Space complexity: O(b*m) for stack storage
    /// * Not complete (may miss solutions due to depth limit)
    /// * Not optimal (may find suboptimal solutions)
    ///
    /// # Implementation Notes
    /// - Uses reversed move ordering to maintain proper DFS exploration order
    /// - Depth-checking prevents redundant exploration of worse paths
    /// - Statistics are updated only when solution is found
    ///
    fn run(&mut self, state: Problem, order: &Order) -> Option<Node> {
        let mut visited = HashMap::<u64, u8>::new();
        let mut stack = Vec::new();
        let current = Node::new(state.get().clone());

        if current.is_solved() {
            return Some(current);
        }

        let dir_order = match order {
            Order::Perm(dirs) => dirs,
            _ => panic!("Podano nieprawidłowy typ Order!"),
        };

        stack.push(current.clone());
        visited.insert(current.hash_code(), current.depth());
        let mut count = 0;

        while let Some(current) = stack.pop() {
            count += 1;

            if current.depth() >= MAX_DEPTH {
                continue;
            }

            for child in current.get_child_nodes_reverse(*dir_order) {
                if let Some(&existing_depth) = visited.get(&child.hash_code()) {
                    if child.depth() >= existing_depth {
                        continue;
                    }
                }

                if child.is_solved() {
                    self.set_visited_count(visited.len());
                    self.set_reached_depth(child.depth() as i16);
                    self.set_result_len(child.depth());
                    self.set_processed_count(count);
                    return Some(child);
                }

                visited.insert(child.hash_code(), child.depth());
                stack.push(child);
            }
        }

        self.set_reached_depth(-1);
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{Algorithm, Dir, Problem, order::Order};

    use super::Dfs;
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
            vec![5, 1, 2, 3],
            vec![0, 6, 7, 4],
            vec![9, 10, 11, 8],   // Correct final state
            vec![13, 14, 15, 12], // Correct final state
        ])
    }

    // Test if BFS finds a solved state immediately
    #[test]
    fn test_dfs_immediate_solution() {
        let solved_state = solved_state();
        let mut dfs = Dfs::new();

        // Since the initial state is solved, BFS should return the solved state immediately
        let result = dfs.run(solved_state.clone(), &Order::Perm(Dir::values()));
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

    // Test if BFS can solve a puzzle
    #[test]
    fn test_dfs_solving_puzzle() {
        let unsolved_state = unsolved_state();
        let mut bfs = Dfs::new();

        // BFS should be able to find the solved state
        let result = bfs.run(unsolved_state.clone(), &Order::Perm(Dir::values()));
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

    // Test BFS on a very small puzzle (e.g., 2x2)
    #[test]
    fn test_dfs_small_puzzle() {
        let small_unsolved_board = Problem::new(vec![
            vec![1, 0],
            vec![3, 2], // Misplaced pieces
        ]);
        let small_solved_board = Problem::new(vec![
            vec![1, 2],
            vec![3, 0], // Correct final state
        ]);
        let unsolved_state = small_unsolved_board;
        let solved_state = small_solved_board;
        let mut bfs = Dfs::new();

        // BFS should return the solved state for the 2x2 puzzle
        let result = bfs.run(unsolved_state, &Order::Perm(Dir::values()));
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

    // Test BFS on an empty board (should panic based on your implementation)
    #[test]
    #[should_panic]
    fn test_bfs_empty_board() {
        let empty_board: Vec<Vec<u8>> = Vec::new();
        let mut bfs = Dfs::new(); // This should panic because the state is empty
        bfs.run(Problem::new(empty_board), &Order::Perm(Dir::values()));
    }
}

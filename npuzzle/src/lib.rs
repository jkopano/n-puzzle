pub mod algorithms;
pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::{
        Dir, Problem,
        order::Order,
        solver::{AStarSolver, BfsSolver, DfsSolver},
    };

    #[test]
    fn test_dfs_solver_finds_solution() {
        let unsolved_board = Problem::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 0],
            vec![9, 10, 11, 8],
            vec![13, 14, 15, 12],
        ]);

        let mut solver = DfsSolver::new(Order::Perm(Dir::values()));
        let solution = solver.solve(unsolved_board.clone());

        println!("solution: {:?}", solution.as_ref().unwrap());

        // Ensure a result is found
        assert!(solution.is_some(), "Solver should find a solution");

        // Ensure result length is valid (non-zero if solved)
        assert!(
            solution.unwrap().result_len > 0,
            "Solution should have a valid length"
        );
    }

    #[test]
    fn test_dfs_solver_performance_metrics() {
        let unsolved_board = Problem::new(vec![
            vec![1, 0, 2, 3],
            vec![5, 10, 7, 4],
            vec![6, 14, 11, 8],
            vec![9, 13, 15, 12],
        ]);

        let mut solver = DfsSolver::new(Order::Perm(Dir::values()));
        let solution = solver.solve(unsolved_board.clone()).unwrap();

        // Check that performance metrics are being recorded
        assert!(
            solution.visited_count > 0,
            "Visited count should be greater than 0"
        );
        assert!(
            solution.processed_count > 0,
            "Processed count should be greater than 0"
        );
        assert!(
            solution.reached_depth >= 0,
            "Reached depth should be non-negative"
        );
    }

    #[test]
    fn test_bfs_solver_finds_solution() {
        let unsolved_board = Problem::new(vec![
            vec![5, 1, 2, 3],
            vec![0, 6, 7, 4],
            vec![9, 10, 11, 8],
            vec![13, 14, 15, 12],
        ]);

        let mut solver = AStarSolver::new(Order::Manh);

        let solution = solver.solve(unsolved_board.clone());
        println!("solution: {:?}", solution.as_ref().unwrap());

        // Ensure a result is found
        assert!(solution.is_some(), "Solver should find a solution");

        // Ensure result length is valid (non-zero if solved)
        assert!(
            solution.unwrap().result_len > 0,
            "Solution should have a valid length"
        );
    }

    #[test]
    fn test_bfs_solver_performance_metrics() {
        let unsolved_board = Problem::new(vec![
            vec![5, 1, 2, 3],
            vec![6, 0, 7, 4],
            vec![9, 10, 11, 8],
            vec![13, 14, 15, 12],
        ]);

        let mut solver = BfsSolver::new(Order::Perm(Dir::values()));
        let solution = solver.solve(unsolved_board.clone()).unwrap();

        // Check that performance metrics are being recorded
        assert!(
            solution.visited_count > 0,
            "Visited count should be greater than 0"
        );
        assert!(
            solution.processed_count > 0,
            "Processed count should be greater than 0"
        );
        assert!(
            solution.reached_depth >= 0,
            "Reached depth should be non-negative"
        );
    }
}

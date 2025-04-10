#![feature(generic_arg_infer)]

use std::fs;

use clap::Parser;
use cmd::command::{Cli, Strategy};
use npuzzle_lib::core::{
    Problem, Solution,
    order::Order,
    solver::{AStarSolver, BfsSolver, DfsSolver},
};

mod cmd;

fn main() {
    let cli = Cli::parse();

    match &cli.strategy {
        Strategy::Bfs {
            order,
            input_file,
            solution_file,
            stats_file,
        } => {
            let problem = load_problem(input_file);
            let mut solver = BfsSolver::new(Order::from((*order).clone()));
            let solution = solver.solve(problem.clone());
            if let Some(solved) = solution {
                write_solution_file(&solved, solution_file);
                write_stats_file(&solved, stats_file);
            } else {
                write_solution_file_err(solution_file);
                write_solution_file_err(stats_file);
            }
        }
        Strategy::Dfs {
            order,
            input_file,
            solution_file,
            stats_file,
        } => {
            let problem = load_problem(input_file);
            let mut solver = DfsSolver::new(Order::from((*order).clone()));
            let solution = solver.solve(problem.clone());
            if let Some(solved) = solution {
                write_solution_file(&solved, solution_file);
                write_stats_file(&solved, stats_file);
            }
        }
        Strategy::Astr {
            heuristic,
            input_file,
            solution_file,
            stats_file,
        } => {
            let problem = load_problem(input_file);
            let mut solver = AStarSolver::new(Order::from((*heuristic).clone()));
            let solution = solver.solve(problem.clone());
            if let Some(solved) = solution {
                write_solution_file(&solved, solution_file);
                write_stats_file(&solved, stats_file);
            }
        }
    }
}

fn write_solution_file(solution: &Solution, path: &str) {
    let mut moves = solution
        .path
        .iter()
        .map(|d| format!("{}", d))
        .collect::<Vec<_>>()
        .join("");

    moves = format!("{}\n{}", solution.result_len, moves);

    fs::write(path, moves).expect("Failed to write solution file");
}

fn write_solution_file_err(path: &str) {
    let moves = format!("{}", -1);

    fs::write(path, moves).expect("Failed to write solution file");
}

fn write_stats_file(solution: &Solution, path: &str) {
    let stats = format!(
        "{}\n{}\n{}\n{}\n{:.3}",
        solution.result_len,
        solution.visited_count,
        solution.processed_count,
        solution.reached_depth,
        solution.duration
    );

    fs::write(path, stats).expect("Failed to write stats file");
}

fn load_problem(input: &str) -> Problem {
    let content = fs::read_to_string(input).expect("Failed to read input file");
    let mut array: Vec<Vec<u8>> = vec![];
    for x in content.lines().skip(1) {
        let v: Vec<u8> = x.split_whitespace().map(|x| x.parse().unwrap()).collect();
        array.push(v);
    }

    Problem::new(array)
}

use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
    path::Path,
};

pub mod helper;

use helper::So;
use npuzzle_lib::core::{
    Order, Problem,
    solver::{AStarSolver, BfsSolver, DfsSolver, SolverEnum},
};

#[derive(Default, Debug)]
struct Temp {
    result_len: usize,
    reached_depth: usize,
    visited_count: usize,
    processed_count: usize,
    duration: f64,
}

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Print out the arguments
    for arg in args.iter() {
        if arg == "norm" {
            let mut solver = SolverEnum::AStar(AStarSolver::new(Order::Manh));
            let x = parse_directory("puzzles", &mut solver);
            println!("ASTAR-MANH:\n{:?}", x.unwrap());

            let mut solver = SolverEnum::AStar(AStarSolver::new(Order::Hamm));
            let x = parse_directory("puzzles", &mut solver);
            println!("ASTAR-Hamm:\n{:?}", x.unwrap());

            let order = Order::from(So::Ludr);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());

            let order = Order::from(So::Lurd);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());

            let order = Order::from(So::Rdul);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());

            let order = Order::from(So::Rdlu);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());

            let order = Order::from(So::Ulrd);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());

            let order = Order::from(So::Uldr);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());

            let order = Order::from(So::Drul);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());

            let order = Order::from(So::Drlu);
            let mut solver = SolverEnum::Bfs(BfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "ludr" {
            let order = Order::from(So::Ludr);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "lurd" {
            let order = Order::from(So::Lurd);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "rdul" {
            let order = Order::from(So::Rdul);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "rdlu" {
            let order = Order::from(So::Rdlu);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "ulrd" {
            let order = Order::from(So::Ulrd);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "uldr" {
            let order = Order::from(So::Uldr);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "drul" {
            let order = Order::from(So::Drul);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        } else if arg == "drlu" {
            let order = Order::from(So::Drlu);
            let mut solver = SolverEnum::Dfs(DfsSolver::new(order.clone()));
            let x = parse_directory("puzzles", &mut solver);
            println!("BFS-{}:\n{:?}", order, x.unwrap());
        }
    }
}

fn load_problem(input: &Path) -> io::Result<Problem> {
    let mut file = fs::File::open(input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let content = fs::read_to_string(input).expect("Failed to read input file");
    let mut array: Vec<Vec<u8>> = vec![];
    for x in content.lines().skip(1) {
        let v: Vec<u8> = x.split_whitespace().map(|x| x.parse().unwrap()).collect();
        array.push(v);
    }

    Ok(Problem::new(array))
}

fn parse_directory(dir_path: &str, solver: &mut SolverEnum) -> io::Result<HashMap<u8, Temp>> {
    let mut temp: HashMap<u8, Temp> = HashMap::new();

    temp.insert(1, Temp::default());
    temp.insert(2, Temp::default());
    temp.insert(3, Temp::default());
    temp.insert(4, Temp::default());
    temp.insert(5, Temp::default());
    temp.insert(6, Temp::default());
    temp.insert(7, Temp::default());

    let mut counter: [u16; 7] = [0, 0, 0, 0, 0, 0, 0];

    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();

        if path.is_file() {
            let p = load_problem(&path).unwrap();
            let s = AStarSolver::new(Order::Manh).solve(p.clone()).unwrap();
            let shuffle_len = s.reached_depth as u8;

            counter[(shuffle_len - 1) as usize] += 1;

            let solution = match solver {
                SolverEnum::Dfs(solver) => solver.solve(p),
                SolverEnum::Bfs(solver) => solver.solve(p),
                SolverEnum::AStar(solver) => solver.solve(p),
            }
            .unwrap();

            if let Some(o) = temp.get_mut(&shuffle_len) {
                o.result_len += solution.result_len as usize;
                o.reached_depth += solution.reached_depth as usize;
                o.visited_count += solution.visited_count;
                o.processed_count += solution.processed_count;
                o.duration += solution.duration;
            }
        }
    }

    for (i, &v) in counter.iter().enumerate() {
        if let Some(o) = temp.get_mut(&((i + 1) as u8)) {
            o.result_len /= v as usize;
            o.reached_depth /= v as usize;
            o.visited_count /= v as usize;
            o.processed_count /= v as usize;
            o.duration /= v as f64;
        }
    }

    Ok(temp)
}

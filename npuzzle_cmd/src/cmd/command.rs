use clap::{Parser, Subcommand, ValueEnum};
use npuzzle_lib::core::{Dir, Order};

#[derive(Parser)]
#[command(
    author = "Jan Banaszkiewicz, Jakub Kopaniewski",
    version = "1.0",
    about = "N-Puzzle program"
)]
pub struct Cli {
    #[command(subcommand)]
    pub strategy: Strategy,
}

#[derive(Subcommand)]
pub enum Strategy {
    Bfs {
        #[arg(value_enum)]
        order: SearchOrder,
        input_file: String,
        solution_file: String,
        stats_file: String,
    },
    Dfs {
        #[arg(value_enum)]
        order: SearchOrder,
        input_file: String,
        solution_file: String,
        stats_file: String,
    },
    Astr {
        #[arg(value_enum)]
        heuristic: HeuristicType,
        input_file: String,
        solution_file: String,
        stats_file: String,
    },
}

#[derive(ValueEnum, Clone, Debug)]
#[value(rename_all = "UPPERCASE")]
pub enum SearchOrder {
    Rdul,
    Rdlu,
    Ruld,
    Rudl,
    Rlud,
    Rldu,
    Drul,
    Drlu,
    Dulr,
    Durl,
    Dlur,
    Dlru,
    Ulrd,
    Uldr,
    Urld,
    Urdl,
    Udlr,
    Udrl,
    Lrud,
    Lrdu,
    Lurd,
    Ludr,
    Ldur,
    Ldru,
}

impl From<HeuristicType> for Order {
    fn from(value: HeuristicType) -> Self {
        match value {
            HeuristicType::Manh => Order::Manh,
            HeuristicType::Hamm => Order::Hamm,
        }
    }
}

impl From<SearchOrder> for Order {
    fn from(search_order: SearchOrder) -> Self {
        match search_order {
            SearchOrder::Rdul => Order::Perm([Dir::Right, Dir::Down, Dir::Up, Dir::Left]),
            SearchOrder::Rdlu => Order::Perm([Dir::Right, Dir::Down, Dir::Left, Dir::Up]),
            SearchOrder::Ruld => Order::Perm([Dir::Right, Dir::Up, Dir::Left, Dir::Down]),
            SearchOrder::Rudl => Order::Perm([Dir::Right, Dir::Up, Dir::Down, Dir::Left]),
            SearchOrder::Rlud => Order::Perm([Dir::Right, Dir::Left, Dir::Up, Dir::Down]),
            SearchOrder::Rldu => Order::Perm([Dir::Right, Dir::Left, Dir::Down, Dir::Up]),
            SearchOrder::Drul => Order::Perm([Dir::Down, Dir::Right, Dir::Up, Dir::Left]),
            SearchOrder::Drlu => Order::Perm([Dir::Down, Dir::Right, Dir::Left, Dir::Up]),
            SearchOrder::Dulr => Order::Perm([Dir::Down, Dir::Up, Dir::Left, Dir::Right]),
            SearchOrder::Durl => Order::Perm([Dir::Down, Dir::Up, Dir::Right, Dir::Left]),
            SearchOrder::Dlur => Order::Perm([Dir::Down, Dir::Left, Dir::Up, Dir::Right]),
            SearchOrder::Dlru => Order::Perm([Dir::Down, Dir::Left, Dir::Right, Dir::Up]),
            SearchOrder::Ulrd => Order::Perm([Dir::Up, Dir::Left, Dir::Right, Dir::Down]),
            SearchOrder::Uldr => Order::Perm([Dir::Up, Dir::Left, Dir::Down, Dir::Right]),
            SearchOrder::Urld => Order::Perm([Dir::Up, Dir::Right, Dir::Left, Dir::Down]),
            SearchOrder::Urdl => Order::Perm([Dir::Up, Dir::Right, Dir::Down, Dir::Left]),
            SearchOrder::Udlr => Order::Perm([Dir::Up, Dir::Down, Dir::Left, Dir::Right]),
            SearchOrder::Udrl => Order::Perm([Dir::Up, Dir::Down, Dir::Right, Dir::Left]),
            SearchOrder::Lrud => Order::Perm([Dir::Left, Dir::Right, Dir::Up, Dir::Down]),
            SearchOrder::Lrdu => Order::Perm([Dir::Left, Dir::Right, Dir::Down, Dir::Up]),
            SearchOrder::Lurd => Order::Perm([Dir::Left, Dir::Up, Dir::Right, Dir::Down]),
            SearchOrder::Ludr => Order::Perm([Dir::Left, Dir::Up, Dir::Down, Dir::Right]),
            SearchOrder::Ldur => Order::Perm([Dir::Left, Dir::Down, Dir::Up, Dir::Right]),
            SearchOrder::Ldru => Order::Perm([Dir::Left, Dir::Down, Dir::Right, Dir::Up]),
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum HeuristicType {
    Manh,
    Hamm,
}

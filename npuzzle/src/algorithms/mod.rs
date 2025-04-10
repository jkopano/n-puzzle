pub mod astar;
pub mod bfs;
pub mod dfs;

pub use astar::AStar;
pub use bfs::Bfs;
pub use dfs::Dfs;

pub const MAX_DEPTH: u8 = 20;

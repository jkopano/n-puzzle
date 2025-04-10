use std::fmt;

use super::Dir;

#[derive(Clone)]
pub enum Order {
    Hamm,
    Manh,
    Perm([Dir; 4]),
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Order::Hamm => write!(f, "Hamm"),
            Order::Manh => write!(f, "Manh"),
            Order::Perm(dirs) => {
                write!(
                    f,
                    "Perm({})",
                    dirs.iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

impl Order {
    pub fn get_heuristic(&self) -> impl Fn(&[u8], u8) -> usize {
        match self {
            // Number of cells not on its correct position
            Order::Hamm => |board: &[u8], depth: u8| {
                depth as usize
                    + board
                        .iter()
                        .enumerate()
                        .filter(|&(i, v)| *v != 0 && *v as usize != i)
                        .count()
            },

            // Sum of distances from solved state
            Order::Manh => |board: &[u8], depth: u8| {
                let dim = board.len().isqrt();
                let mut manhattan_cost = 0;

                // Calculate base Manhattan distance
                for (index, &value) in board.iter().enumerate() {
                    if value != 0 {
                        let target_pos = (value - 1) as usize;
                        let target_row = target_pos / dim;
                        let target_col = target_pos % dim;
                        let current_row = index / dim;
                        let current_col = index % dim;

                        manhattan_cost +=
                            target_row.abs_diff(current_row) + target_col.abs_diff(current_col);
                    }
                }

                depth as usize + manhattan_cost
            },
            Order::Perm(_) => panic!("Perm variant does not return a heuristic function!"),
        }
    }
}

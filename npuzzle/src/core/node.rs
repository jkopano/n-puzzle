use core::fmt;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::core::Dir;

// Represents a puzzle state
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Node {
    board: Vec<u8>,
    dim: u8,
    parent: Option<Rc<Node>>,
    depth: u8,
    next_move: Option<Dir>,
    hash_code: u64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.depth().cmp(&self.depth)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.dim {
            for col in 0..self.dim {
                write!(f, "{:2} ", self.board[(row * self.dim + col) as usize])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Node {
    /// Constructs a new `Node` instance from a 2D board representation.
    ///
    /// # Arguments
    /// * `vec` - 2D vector representing the puzzle board state
    ///
    /// # Panics
    /// Panics if the board is invalid.
    pub fn new(vec: Vec<Vec<u8>>) -> Self {
        let board = vec.into_iter().flatten().collect::<Vec<u8>>();

        if !Self::is_board_valid(&board) {
            panic!("Board is Invalid")
        }

        Self {
            hash_code: Node::make_hash(&board),
            dim: board.len().isqrt() as u8,
            board,
            ..Self::default()
        }
    }

    fn with_parent(board: Vec<u8>, parent: &Node, dir: Dir) -> Result<Self, String> {
        if !Self::is_board_valid(&board) {
            return Err("Invalid board".to_string());
        }
        Ok(Self {
            hash_code: Node::make_hash(&board),
            board,
            dim: parent.dim,
            depth: parent.depth + 1,
            parent: Some(Rc::new(parent.clone())),
            next_move: Some(dir),
        })
    }

    /// Generates a default (solved) board configuration for given size.
    ///
    /// # Arguments
    /// * `size` - The dimension of the board (e.g., 3 for 3x3 puzzle)
    ///
    /// # Returns
    /// * 2D vector representing the solved state
    pub fn default_board(size: u8) -> Vec<Vec<u8>> {
        let dim = size;
        (1..=dim * dim - 1)
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>()
            .chunks(dim as usize)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn is_board_valid(board: &[u8]) -> bool {
        let mut temp: Vec<u8> = board.to_vec();
        if temp.is_empty() {
            return false;
        }

        temp.sort_unstable();

        temp.iter().enumerate().all(|(i, x)| *x == i as u8)
    }

    /// Checks if the current board state represents a solved puzzle.
    ///
    /// # Returns
    /// * `true` if the board is in solved state, `false` otherwise
    pub fn is_solved(&self) -> bool {
        let mut expected: Vec<u8> = (1..=(self.dim * self.dim)).collect();
        expected.pop();
        expected.push(0);

        self.board == expected
    }

    fn find_zero(&self) -> u8 {
        self.board.iter().position(|&x| x == 0).unwrap() as u8
    }

    fn make_hash(board: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();

        board.hash(&mut hasher);

        hasher.finish()
    }

    fn can_move(&self, direction: Dir) -> bool {
        let empty_field = self.find_zero();
        match direction {
            Dir::Left => empty_field % self.dim > 0,
            Dir::Right => empty_field % self.dim < (self.dim - 1),
            Dir::Down => empty_field / self.dim < (self.dim - 1),
            Dir::Up => empty_field / self.dim > 0,
        }
    }
    pub fn get_board(&self) -> &Vec<u8> {
        &self.board
    }

    pub fn get_dim(&self) -> u8 {
        (self.board.len() as f64).sqrt() as u8
    }

    /// Gets all valid moves from current state in specified priority order.
    ///
    /// # Arguments
    /// * `order` - Array of 4 `Dir` values specifying move priority order
    ///
    /// # Returns
    /// * Vector of valid directions that can be moved from current state
    pub fn get_valid_moves(&self, order: [Dir; 4]) -> Vec<Dir> {
        order
            .into_iter()
            .filter(|dir| self.can_move(*dir))
            .collect()
    }

    /// Generates a new node by moving the empty space in specified direction.
    ///
    /// # Arguments
    /// * `dir` - Direction to move the empty space
    ///
    /// # Returns
    /// * `Some(Node)` if move is valid, `None` otherwise
    pub fn get_node_for_move(&self, dir: Dir) -> Option<Self> {
        if !self.can_move(dir) {
            return None;
        }

        let new_pos = match dir {
            Dir::Left => self.find_zero() - 1,
            Dir::Right => self.find_zero() + 1,
            Dir::Up => self.find_zero() - self.dim,
            Dir::Down => self.find_zero() + self.dim,
        };

        let mut new_board = self.board.clone();
        let zero_index = self.find_zero() as usize;
        let new_index = new_pos as usize;

        new_board.swap(zero_index, new_index);

        Some(Node::with_parent(new_board, self, dir).unwrap())
    }

    pub fn hash_code(&self) -> u64 {
        self.hash_code
    }

    pub fn next_move(&self) -> Option<Dir> {
        self.next_move
    }

    pub fn parent(&self) -> &Option<Rc<Self>> {
        &self.parent
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn dir_iter(&self) -> NodePathIterator {
        NodePathIterator {
            current: Some(Rc::new(self.clone())),
        }
    }

    /// Gets all child nodes (possible next states) in specified move order.
    ///
    /// # Arguments
    /// * `order` - Array of 4 `Dir` values specifying move priority order
    ///
    /// # Returns
    /// * Vector of child nodes
    pub fn get_child_nodes(&self, order: [Dir; 4]) -> Vec<Node> {
        self.get_valid_moves(order)
            .into_iter()
            .filter_map(|dir| self.get_node_for_move(dir))
            .collect()
    }

    /// Gets all child nodes in reverse order of specified move priorities.
    ///
    /// # Arguments
    /// * `order` - Array of 4 `Dir` values specifying move priority order
    ///
    /// # Returns
    /// * Vector of child nodes in reverse order
    pub fn get_child_nodes_reverse(&self, order: [Dir; 4]) -> Vec<Node> {
        let mut nodes = self.get_child_nodes(order);

        nodes.reverse();

        nodes
    }
}

pub struct NodePathIterator {
    pub current: Option<Rc<Node>>,
}

impl Iterator for NodePathIterator {
    type Item = Dir;

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = self.current.take();

        if let Some(state) = current_state {
            let dir = state.next_move();

            self.current = state.parent().as_ref().map(|p| p.clone());

            return dir;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Dir, node::Node};

    #[test]
    fn test_new_valid_board() {
        let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
        let state = Node::new(board.clone());
        assert_eq!(
            *state.get_board(),
            board.into_iter().flatten().collect::<Vec<u8>>()
        );
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_board() {
        let board = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9], // Invalid: 9 is out of range
        ];
        Node::new(board);
    }

    #[test]
    fn test_find_zero() {
        let board = vec![vec![1, 2, 3], vec![4, 0, 5], vec![6, 7, 8]];
        let state = Node::new(board);
        assert_eq!(state.find_zero(), 4);
    }

    #[test]
    fn test_get_valid_moves() {
        let board = vec![
            vec![0, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 1], // Zero at (2,2)
        ];
        let state = Node::new(board);
        let moves = state.get_valid_moves(Dir::values());

        assert!(moves.contains(&Dir::Down)); // Can move left
        assert!(moves.contains(&Dir::Right)); // Can move up
        assert_eq!(moves.len(), 2); // Only two valid moves
    }

    #[test]
    fn test_get_state_for_move() {
        let board = vec![
            vec![0, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 1], // Zero at (2,2)
        ];
        let state = Node::new(board);

        // Move left
        if let Some(new_state) = state.get_node_for_move(Dir::Right) {
            let expected_board = vec![
                vec![2, 0, 3],
                vec![4, 5, 6],
                vec![7, 8, 1], // Zero moved left
            ];
            assert_eq!(
                *new_state.get_board(),
                expected_board.into_iter().flatten().collect::<Vec<u8>>()
            );
        } else {
            panic!("Expected a valid state after move");
        }
    }

    #[test]
    fn test_get_child_states() {
        let board = vec![
            vec![1, 2, 3],
            vec![4, 0, 5], // Zero at (1,1)
            vec![6, 7, 8],
        ];
        let state = Node::new(board);
        let child_states = state.get_child_nodes(Dir::values());

        assert_eq!(child_states.len(), 4); // Can move up, down, left, right
    }

    #[test]
    fn test_is_solved() {
        let solved_board = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 0], // Correct final state
        ];
        let unsolved_board = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 0, 8], // Wrong position
        ];
        let solved_state = Node::new(solved_board);
        let unsolved_state = Node::new(unsolved_board);
        assert!(solved_state.is_solved());
        assert!(!unsolved_state.is_solved());
    }
    #[test]
    fn test_valid_moves_initial_state() {
        // Create initial state with zero at (0,0)
        let state = Node::new(vec![
            vec![0, 1, 2, 11],
            vec![3, 4, 5, 12],
            vec![6, 7, 8, 13],
            vec![10, 9, 15, 14],
        ]);

        // Expected valid moves: Down, Right
        let expected = vec![Dir::Down, Dir::Right];
        assert_eq!(state.get_valid_moves(Dir::values()), expected);
    }

    #[test]
    fn test_get_valid_moves_from_top_left() {
        // Arrange
        let board = vec![vec![0, 2, 3], vec![4, 5, 6], vec![7, 8, 1]];
        let state = Node::new(board); // Act
        let moves = state.get_valid_moves(Dir::values());

        // Assert
        assert!(moves.contains(&Dir::Down), "Should be able to move Down");
        assert!(moves.contains(&Dir::Right), "Should be able to move Right");
        assert_eq!(moves.len(), 2, "Expected two valid moves");
    }

    #[test]
    fn test_get_valid_moves_from_bottom_right() {
        // Arrange
        let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
        let state = Node::new(board);

        // Act
        let moves = state.get_valid_moves(Dir::values());

        // Assert
        assert!(moves.contains(&Dir::Up), "Should be able to move Up");
        assert!(moves.contains(&Dir::Left), "Should be able to move Left");
        assert_eq!(moves.len(), 2, "Expected two valid moves");
    }

    #[test]
    fn test_get_valid_moves_from_center() {
        // Arrange
        let board = vec![vec![1, 2, 3], vec![4, 0, 5], vec![6, 7, 8]];
        let state = Node::new(board);
        // Act
        let moves = state.get_valid_moves(Dir::values());

        // Assert
        assert!(moves.contains(&Dir::Up), "Should be able to move Up");
        assert!(moves.contains(&Dir::Down), "Should be able to move Down");
        assert!(moves.contains(&Dir::Left), "Should be able to move Left");
        assert!(moves.contains(&Dir::Right), "Should be able to move Right");
        assert_eq!(moves.len(), 4, "Expected four valid moves");
    }

    #[test]
    fn test_get_state_for_move_right() {
        // Arrange
        let board = vec![vec![0, 2, 3], vec![4, 5, 6], vec![7, 8, 1]];
        let state = Node::new(board);

        // Act
        let new_state = state.get_node_for_move(Dir::Right).unwrap();

        // Assert
        let expected_board = vec![vec![2, 0, 3], vec![4, 5, 6], vec![7, 8, 1]]
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>();
        assert_eq!(
            *new_state.get_board(),
            expected_board,
            "Zero should have moved right"
        );
        assert_eq!(
            new_state.parent.as_ref().unwrap().board,
            state.board,
            "Parent should be the original state"
        );
        assert_eq!(
            new_state.next_move,
            Some(Dir::Right),
            "Next move should be Right"
        );
    }

    #[test]
    fn test_get_state_for_move_invalid() {
        // Arrange
        let board = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let state = Node::new(board);

        // Act
        let new_state = state.get_node_for_move(Dir::Left);

        // Assert
        assert!(
            new_state.is_none(),
            "Moving left from the left edge should return None"
        );
    }

    #[test]
    fn test_make_hash() {
        // Arrange
        let board1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let board2 = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];

        // Act
        let hash1 = Node::make_hash(&board1);
        let hash2 = Node::make_hash(&board2);

        // Assert
        assert_ne!(
            hash1, hash2,
            "Different boards should produce different hashes"
        );

        // Test that the same board produces the same hash
        let board3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let hash3 = Node::make_hash(&board3);
        assert_eq!(hash1, hash3, "Same boards should produce the same hash");
    }

    #[test]
    fn test_with_parent() {
        // Arrange
        let initial_state = Node::new(vec![vec![1, 2, 3], vec![4, 0, 5], vec![6, 7, 8]]);

        // Act
        let new_board = vec![1, 2, 3, 0, 4, 5, 6, 7, 8];
        let moved_state = Node::with_parent(new_board, &initial_state, Dir::Left).unwrap();

        // Assert
        assert_eq!(
            moved_state.parent.unwrap().board,
            initial_state.board,
            "Parent board should match"
        );
        assert_eq!(moved_state.depth, 1, "Depth should be incremented");
        assert_eq!(
            moved_state.next_move,
            Some(Dir::Left),
            "Next move should be set"
        );
    }
}

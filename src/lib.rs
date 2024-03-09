use std::collections::HashSet;

use itertools::Itertools;

pub struct Board {
    queens: HashSet<Coord>,
    width: usize,
    height: usize,
}

pub type Coord = (usize, usize);

impl Board {
    pub fn square(size: usize, queens: Vec<Coord>) -> Self {
        Board::rectangle(size, size, queens)
    }

    pub fn rectangle(width: usize, height: usize, queens: Vec<Coord>) -> Self {
        Board {
            width,
            height,
            queens: queens.iter().cloned().collect(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_full() {
            return false;
        }

        let n_queens = self.queens.len();
        let distinct_rows = self.count_distinct(|q| q.0 as i64);
        let distinct_cols = self.count_distinct(|q| q.1 as i64);

        // Coordinates on same primary diagonal has the same value of y+x
        let distinct_pos_diagonal = self.count_distinct(|q| q.0 as i64 + q.1 as i64);
        // Coordinates on same seconday diagonal has the same value of y-x
        let distinct_neg_diagonal = self.count_distinct(|q| q.0 as i64 - q.1 as i64);

        if distinct_cols != n_queens
            || distinct_rows != n_queens
            || distinct_pos_diagonal != n_queens
            || distinct_neg_diagonal != n_queens
        {
            return false;
        }
        true
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.has_queen((y, x)) {
                    print!("Q ");
                } else {
                    print!(". ");
                }
            }
            print!("\n");
        }
    }

    fn has_queen(&self, coord: Coord) -> bool {
        self.queens.contains(&coord)
    }

    fn count_distinct<F>(&self, f: F) -> usize
    where
        F: Fn(Coord) -> i64,
    {
        self.queens.iter().map(|q| f(*q)).unique().count()
    }

    fn is_full(&self) -> bool {
        // The board is "full" when tha maximum number of queens has been positioned
        let expected_queens = self.width.min(self.height);
        return self.queens.len() == expected_queens;
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn board_1by1_isvalid() {
        let queens = vec![(0, 0)];
        let board = Board::square(1, queens);
        assert_eq!(true, board.is_valid());
    }

    #[test]
    fn board_2by2_isnotvalid() {
        let queens = vec![(0, 0), (1, 1)];
        let board = Board::square(2, queens);
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_4x4_with_two_queens_is_invalid() {
        let board = Board::square(4, vec![(0, 0), (1, 2)]);
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_4x4_valid() {
        let queens = vec![(0, 1), (1, 3), (2, 0), (3, 2)];
        let board = Board::square(4, queens);
        assert_eq!(true, board.is_valid());
    }

    #[test]
    fn board_4x4_same_row_invalid() {
        let queens = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
        let board = Board::square(4, queens);
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_4x4_same_col_invalid() {
        let queens = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        let board = Board::square(4, queens);
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_4x4_same_diagonal_positive_invalid() {
        let queens = vec![(0, 1), (1, 2), (2, 0), (3, 3)];
        let board = Board::square(4, queens);
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_4x4_same_diagonal_negative_invalid() {
        let queens = vec![(0, 3), (1, 1), (2, 0), (3, 2)];
        let board = Board::square(4, queens);
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_8x8_valid() {
        let queens = vec![
            (0, 4),
            (1, 1),
            (2, 3),
            (3, 6),
            (4, 2),
            (5, 7),
            (6, 5),
            (7, 0),
        ];
        let board = Board::square(8, queens);
        assert_eq!(true, board.is_valid());
    }
}

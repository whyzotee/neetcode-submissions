use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for r in 0..9 {
            for c in 0..9 {
                let value = board[r][c];
                if value == '.' {
                    continue;
                }

                if let Some(col) = cols.get(&c) {
                    if col.contains(&value) {
                        return false;
                    }
                }

                if let Some(row) = rows.get(&r) {
                    if row.contains(&value) {
                        return false;
                    }
                }

                if let Some(square) = squares.get(&(c / 3, r / 3)) {
                    if square.contains(&value) {
                        return false;
                    }
                }

                cols.entry(c).or_insert_with(HashSet::new).insert(value);
                rows.entry(r).or_insert_with(HashSet::new).insert(value);
                squares
                    .entry((c / 3, r / 3))
                    .or_insert_with(HashSet::new)
                    .insert(value);
            }
        }

        true
    }
}

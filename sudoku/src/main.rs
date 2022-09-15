fn main() {
    let initial_grid: Grid = Grid::new([
        [0, 4, 3, 0, 0, 0, 0, 0, 9], 
        [0, 0, 0, 6, 0, 0, 0, 0, 5], 
        [0, 0, 0, 0, 0, 4, 1, 0, 0], 
        [9, 0, 1, 0, 5, 0, 0, 0, 0], 
        [0, 0, 0, 7, 2, 6, 0, 0, 0], 
        [0, 0, 8, 0, 1, 0, 0, 0, 0], 
        [0, 1, 0, 0, 0, 0, 7, 2, 0], 
        [7, 0, 0, 0, 0, 0, 0, 0, 0], 
        [2, 0, 0, 0, 0, 5, 0, 6, 0], 
    ]);
    let result = initial_grid.solve_sudoku();
}

const ALL_EMPTY: u16 = 0b111_111_111_0;

#[derive(Copy, Clone)]
struct Grid {
    grid: [[i8; 9]; 9],
    rows: [u16; 9],
    cols: [u16; 9],
    boxes: [[u16; 3]; 3],
}

impl Grid {
    fn new(grid: [[i8; 9]; 9]) -> Self {
        let mut result = Grid {
            grid,
            rows: [ALL_EMPTY; 9],
            cols: [ALL_EMPTY; 9],
            boxes: [[ALL_EMPTY; 3]; 3],
        };
        for row in 0..9 {
            for col in 0..9 {
                if result.grid[row][col] != 0 {
                    result.set_occupied(row, col, result.grid[row][col]);
                }
            }
        }
        result
    }

    fn set_occupied(&mut self, row: usize, col: usize, val: i8) {
        self.grid[row][col] = val;
        self.rows[row] &= !(1 << val as u8);
        self.cols[col] &= !(1 << val as u8);
        self.boxes[row / 3][col / 3] &= !(1 << val as u8);
    }

    fn set_vacant(&mut self, row: usize, col: usize) {
        let val = self.grid[row][col];
        self.rows[row] |= 1 << val as u8;
        self.cols[col] |= 1 << val as u8;
        self.boxes[row / 3][col / 3] |= 1 << val as u8;
        self.grid[row][col] = 0;
    }

    fn find_empty(&self) -> (usize, usize) {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    return (row, col)
                }
            }
        }
        print!("Done");
        return (9,9)
    }

    fn solve_sudoku(mut self) -> bool {
        let l: (usize, usize) = self.find_empty();
        if l == (9, 9) {
            self.print();
            return true
        }
        for i in 1..10 {
            if self.is_location_safe(l.0, l.1, i) {
                self.set_occupied(l.0, l.1, i);
                if self.solve_sudoku() {
                    return true;
                }
                self.set_vacant(l.0, l.1);
            }
        }
        return false
    }

    fn is_location_safe(&self, row: usize, col: usize, num: i8) -> bool {
        let box_vacancy = self.boxes[row / 3][col / 3];
        let row_vacancy = self.rows[row];
        let col_vacancy = self.cols[col];
        box_vacancy & row_vacancy & col_vacancy & (1 << num as u8) != 0
    }

    fn print(&self) {
        println!();
        for row in self.grid {
            for item in row {
                print!("{:?} ", item);
            }
            println!();
        }
    }
}

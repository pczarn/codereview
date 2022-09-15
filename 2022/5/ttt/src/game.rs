use std::collections::HashMap;
use std::io;

const BOARD_SIZE: usize = 9;
const P1: char = 'X';
const P2: char = 'O';
pub struct Ttt {
    pub board: [char; BOARD_SIZE],
    pub active_player: char,
    pub game_over: bool,
    pub winner: char,
}

impl Ttt {
    pub fn new() -> Ttt {
        let board = ['0', '1', '2', '3', '4', '5', '6', '7', '8'];
        Ttt {
            board,
            active_player: P1,
            game_over: false,
            winner: ' ',
        }
    }

    pub fn play(&mut self) -> char {
        while !self.game_over {
            self.print_board();

            let pick = self.get_input();
            self.board[pick] = self.active_player;

            self.game_over = self.is_game_over();

            if self.game_over {
                self.winner = self.active_player;
            } else {
                self.switch_player();
            }
        }

        self.print_board();
        println!("\n{} is the winner!", self.winner);
        return self.winner;
    }

    pub fn is_game_over(&self) -> bool {
        if self.vacant_coords().count() == 0 {
            println!("It's a tie!");
            return true;
        }
        if self.board[0] == self.board[1] && self.board[0] == self.board[2] {
            println!("Top row win!");
            return true;
        }
        if self.board[3] == self.board[4] && self.board[3] == self.board[5] {
            println!("Middle row win!");
            return true;
        }
        if self.board[6] == self.board[7] && self.board[6] == self.board[8] {
            println!("Bottom row win!");
            return true;
        }
        if self.board[0] == self.board[3] && self.board[0] == self.board[6] {
            println!("Left column win!");
            return true;
        }
        if self.board[1] == self.board[4] && self.board[1] == self.board[7] {
            println!("Center column win!");
            return true;
        }
        if self.board[2] == self.board[5] && self.board[2] == self.board[8] {
            println!("Right column win!");
            return true;
        }
        if self.board[0] == self.board[4] && self.board[0] == self.board[8] {
            println!("Backslash win!");
            return true;
        }
        if self.board[2] == self.board[4] && self.board[2] == self.board[6] {
            println!("Forwardslash win!");
            return true;
        }

        return false;
    }

    pub fn switch_player(&mut self) {
        if self.active_player == P1 {
            self.active_player = P2;
        } else {
            self.active_player = P1;
        }
    }

    pub fn print_board(&self) {
        println!(
            " {} | {} | {} ",
            self.board[0], self.board[1], self.board[2]
        );
        println!("-----------");
        println!(
            " {} | {} | {} ",
            self.board[3], self.board[4], self.board[5]
        );
        println!("-----------");
        println!(
            " {} | {} | {} ",
            self.board[6], self.board[7], self.board[8]
        );
    }

    pub fn get_input_ML(&self, choice: usize) -> usize {
        return choice;
    }

    fn vacant_coords(&self) -> impl Iterator<Item=usize> + '_ {
        self.board.iter().enumerate().filter_map(|(i, &ch)| {
            if ch == P1 || ch == P2 {
                None
            } else {<
                Some(i)
            }
        })
    }

    pub fn get_input(&self) -> usize {
        println!("Choose a selection from the grid.");
        loop {
            let mut coord = String::new();
            io::stdin()
                .read_line(&mut coord)
                .expect("Failed to read line");
            let coord: usize = match coord.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid selection.");
                    continue;
                }
            };
            if !self.vacant_coords().find(|&i| i == coord).is_some() {
                println!("Please enter a valid selection.");
                continue;
            }
            return coord;
        }
    }
}
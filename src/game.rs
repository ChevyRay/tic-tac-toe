use std::{cmp::max, usize};
use text_io::read;
use Mark::Circle;
use Mark::Cross;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mark {
    Circle = -1,
    Cross = 1,
}

pub struct Game {
    pub board: [Option<Mark>; 9],
    pub turn: Mark,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: [None; 9],
            turn: Circle,
        }
    }
}

impl Game {
    pub fn clear_board(&mut self) {
        self.board = [None; 9];
    }
    fn print(&self) {
        println!("-------------");
        for i in 0..9 {
            let icon = match self.board[i] {
                Some(mark) => match mark {
                    Circle => 'O',
                    Cross => 'X',
                },
                None => ' ',
            };
            print!("| {} ", icon);

            if (i + 1) % 3 == 0 {
                println!("|\n-------------");
            }
        }
    }
    fn set_tile(&mut self, cord: usize) {
        assert!(cord < 9);
        if self.board[cord].is_none() {
            self.board[cord] = Some(self.turn);
            self.turn = match self.turn {
                Circle => Cross,
                Cross => Circle,
            };
        } else {
            println!("This tile is already occupied try again!");
        }
    }
    fn is_game_ended(&self) -> Option<Mark> {
        // All possible winning lines
        [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ]
        .iter()
        .find_map(|line| {
            if line.iter().all(|&i| self.board[i] == Some(Circle)) {
                Some(Circle)
            } else if line.iter().all(|&i| self.board[i] == Some(Cross)) {
                Some(Cross)
            } else {
                None
            }
        })
    }
    fn get_tile(&mut self) {
        println!("Please enter a tile: ");
        let i: usize = read!();
        self.set_tile(i - 1);
    }
    pub fn run_game(&mut self) {
        self.print();

        while self.is_game_ended() == None {
            self.get_tile();
            println!("\n");
            self.print();
        }

        match self.is_game_ended() {
            Some(winner) => match winner {
                Circle => println!(r#"Circle Wins!"#),
                Cross => println!(r#"Cross Wins!"#),
            },
            None => println!(r#"Draw!"#),
        }
    }
}
impl Game {
    fn evaluate(&self) -> i32 {
        let result = self.is_game_ended();
        match result {
            Some(mark) => match mark {
                Circle => -5000,
                Cross => 5000,
            },
            None => 0,
        }
    }
    fn get_legal_moves(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| tile.map_or(Some(i), |_| None))
            .collect()
    }
    fn takeback(&mut self, cord: usize) {
        if self.board[cord].is_some() {
            self.board[cord] = None;
            self.turn = match self.turn {
                Circle => Cross,
                Cross => Circle,
            };
        } else {
            println!("Tile is already empty!")
        }
    }
    fn negamax(&mut self) -> i32 {
        if self.is_game_ended().is_some() {
            self.evaluate() * self.turn as i32
        } else {
            let mut v = -1000000;
            let legal_moves = self.get_legal_moves();
            for legal_move in legal_moves.iter() {
                self.set_tile(*legal_move);
                v = max(v, -self.negamax());
                self.takeback(*legal_move);
            }
            v
        }
    }
    fn ai_play(&mut self) {
        let mut v = -1000000;
        let legal_moves = self.get_legal_moves();
        let mut best_move: usize = 0;
        for legal_move in legal_moves.iter() {
            self.set_tile(*legal_move);
            let score = -self.negamax();
            self.takeback(*legal_move);
            if score > v {
                best_move = *legal_move;
                v = score
            }
        }
        self.set_tile(best_move);
    }
    pub fn run_game_vs_ai(&mut self) {
        println!("Do you want to go first or second(type 1 or 2): ");
        let i: i8 = read!();
        self.print();
        while self.is_game_ended() == None {
            if i == 1 {
                self.get_tile();
                println!("\n");
                self.print();
            }
            if self.is_game_ended() != None {
                break;
            }
            self.ai_play();
            println!("\n");
            self.print();
            if i == 2 && self.is_game_ended() == None {
                self.get_tile();
                println!("\n");
                self.print();
            }
        }
        match self.is_game_ended() {
            Some(winner) => match winner {
                Circle => println!(r#"Circle Wins!"#),
                Cross => println!(r#"Cross Wins!"#),
            },
            None => println!("error!"),
        }
    }
}

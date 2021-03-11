use game::Game;
use text_io::read;

extern crate text_io;
mod game;
fn main() {
    let mut quit = false;
    let mut game = Game::new();
    while !quit {
        println!("Multiplayer or Ai(M/A): ");
        let game_mode: char = read!();
        if game_mode == 'M' || game_mode == 'm' {
            game.start_game();
        } else if game_mode == 'A' || game_mode == 'a' {
            game.start_game_vs_ai();
        }
        println!("Do you want to quit?(Y/N)");
        let is_quiting: char = read!();
        if is_quiting == 'Y' || is_quiting == 'y' {
            quit = true;
        }
        game.clear_board();
    }
}
#[test]
fn test_negamax() {
    let mut game = Game::new();
    game.start_game_vs_ai();
}

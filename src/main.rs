use game::Game;
use text_io::read;

extern crate text_io;
mod game;
fn main() {
    let mut game = Game::default();
    loop {
        println!("Multiplayer or Ai(M/A): ");
        match read!() {
            'M' | 'm' => game.run_game(),
            'A' | 'a' => game.run_game_vs_ai(),
            _ => {}
        }
        println!("Do you want to quit?(Y/N)");
        match read!() {
            'Y' | 'y' => break,
            _ => {}
        }
        game.clear_board();
    }
}
#[test]
fn test_negamax() {
    let mut game = Game::new();
    game.run_game_vs_ai();
}

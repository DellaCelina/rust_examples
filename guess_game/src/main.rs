extern crate rand;

/// Guess game module
mod guess_game;

fn main() {
    use guess_game::*;

    let game_status = GameStatus::new();
    game_status.gaming();
}

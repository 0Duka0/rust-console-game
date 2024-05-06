mod game;
mod board;
mod r#enum;
mod player;
mod cpu;
use crate::game::OthelloGame;
use crate::player::{Player};

fn main() {
    loop {
        let mut player = Player::new();
        let mut game = OthelloGame::new(player);
        game.configure();
        game.start();
        game.results();
        if !game.continueOrNot() {
            break;
        }
    }
}
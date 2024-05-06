use std::{time::SystemTime, io};
use crate::r#enum::{Color, Type};

pub struct Player {
  playerType: Type,
  playerColor: Color,
}

impl Player {
  pub fn new() -> Self {
    Player {
      playerType: Type::Human,
      playerColor: Color::Black,
    }
  }

  pub fn  changeColor (&mut self, color: Color) {
    self.playerColor = color
  }

  pub fn input(moves: usize) -> usize {
    loop {
        let mut input = String::new();
        println!("指手の数字を入力してください↓");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(n) = input.trim().parse::<usize>() {
                    if n < 1 || n > moves {
                        println!("1~{}の数字を入力してください", moves);
                        continue;
                    } else {
                        return n;
                    }
                }
            }
            Err(_) => {
                println!("1~{}の数字を入力してください", moves);
            }
        }
    }
  }
}

use std::{time::SystemTime, io};

use crate::board::{Board, GameResult};
use crate::r#enum::{Color, Type};
use crate::player::{Player};
use crate::cpu::{Cpu};


#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum OthelloGameState {
    BeforeMatch,
    InMatch,
    MatchFinished,
}

pub struct OthelloGame {
    state: OthelloGameState,
    blackPlayer: Type,
    board: Board,
    player: Player,
}

impl OthelloGame {
    const blackLABEL: &'static str = &"黒[●]";
    const whiteLABEL: &'static str = &"白[○]";
    const humanLABEL: &'static str = &"あなた";
    const cpuLABEL: &'static str = &"CPU";

    pub fn new(player: Player) -> Self {
        OthelloGame {
            state: OthelloGameState::BeforeMatch,
            blackPlayer: Type::Human,
            board: Board::new(),
            player: player,
        }
    }

    pub fn configure(&mut self) {
        let blackL = OthelloGame::blackLABEL;
        let whiteL = OthelloGame::whiteLABEL;
        let humanL = OthelloGame::humanLABEL;
        let cpuL = OthelloGame::cpuLABEL;
        loop {
            let mut input = String::new();
            println!("現在の設定は、{}: {} , {}: {}", blackL, humanL, whiteL, cpuL);
            println!("黒番・白番を入れ替えますか？ 1: 入れ替える, 2: そのまま");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Ok(n) = input.trim().parse::<usize>() {
                        if n <= 0 && n > 2 {
                            println!("1 or 2 を入力してください");
                            continue;
                        } else {
                            if n == 1 {
                              self.player.changeColor(Color::White);
                              self.blackPlayer = Type::Cpu;
                            }
                            return;
                        }
                    }
                }
                Err(_) => {
                    println!("1 or 2 を入力してください");
                }
            }
        }
    }

    pub fn start(&mut self) {
        let mut pass = false;
        let rowLabel = ["H","G","F","E","D","C","B","A"];
        loop {
            self.board.show();
            let movesValue = self.board.legalMoves();
            let moves = self.board.split_moves(movesValue);
            let blackTurn = self.board.turn() == Color::Black;
            let pLabel = if blackTurn {"黒[●]"} else {"白[○]"};
            
            if moves.len() != 0 {
                println!("{}手目 - {} の手番です", self.board.turns() + 1, pLabel);
                let (v_black, v_white) = self.board.values();
                println!("黒評価値: {}, 白評価値: {}", v_black, v_white);
                println!("Moves");
                for (i, m) in moves.iter().enumerate() {
                    let n_shift = format!("{:b}", m).len()-1;
                    println!("{}: {}{}", i+1, rowLabel[n_shift%8], 8-n_shift/8);
                }
                let input = if blackTurn {
                    let human = self.blackPlayer == Type::Human;
                    if human {Player::input} else {OthelloGame::cpuInput}
                } else {
                    let human = self.blackPlayer == Type::Cpu;
                    if human {Player::input} else {OthelloGame::cpuInput}
                };
                let index = input(moves.len());
                self.board = self.board.reverse(moves[index-1]);
                pass = false;
                if self.board.end() {
                    self.state = OthelloGameState::MatchFinished;
                    self.board.show();
                    return;
                }
            } else {
                println!("{} の手番ですが指す手がないためパスします", pLabel);
                self.board = self.board.pass();
                if pass {
                    self.state = OthelloGameState::MatchFinished;
                    self.board.show();
                    return;
                }
                pass = true;
            }
        }
    }

    pub fn results(&self) {
        let winner = self.board.judge();
        match winner {
            GameResult::Winner(player) => {
                match player {
                  Color::Black => println!("黒[●] の勝ちです。"),
                  Color::White => println!("白[○] の勝ちです。"),
                }
            }
            GameResult::Draw => println!("引き分けです。"),
        }
    }

    pub fn continueOrNot(&self) -> bool {
        loop {
            let mut input = String::new();
            println!("続ける or 終了する？ 1: 続ける, 2: 終了する");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Ok(n) = input.trim().parse::<usize>() {
                        if n <= 0 && n > 2 {
                            println!("1 or 2 を入力してください");
                            continue;
                        } else {
                            return n == 1;
                        }
                    }
                }
                Err(_) => {
                    println!("1 or 2 を入力してください");
                }
            }
        }
    }
    fn cpuInput(n: usize) -> usize {
      let timestamp = SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .expect("Failed to obtain timestamp")
          .as_nanos();
      (timestamp as usize % n) + 1
    }
}
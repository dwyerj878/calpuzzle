use std::vec;

mod piece;
mod game;
mod tile;
mod board;

use crate::game::Game;
use crate::piece::Piece;
use crate::board::Board;

fn main() {
    
    // Create the board
    let mut b = Board::new();
    
    b.reserve(String::from("Mar"));
    b.reserve(String::from("27"));

    b.draw();

    // create the pieces    
    let pieces = create_pieces(); 

    for idx in 0 .. pieces.len() {
        println!("{:?}", pieces[idx]);
        pieces[idx].draw()
    }

    play(b, pieces);
}

fn create_pieces() -> [Piece; 8] {
    return [
    Piece {id : 0, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1]], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 1, shape : [[0,0], [0,1], [1,0], [2,0], [2,1], [2,1]], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 2, shape : [[0,0], [0,1], [1,1], [2,1], [2,2], [2,2]], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 3, shape : [[1,0], [1,1], [1,2], [0,2], [0,3], [0,3]], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 4, shape : [[1,0], [0,1], [1,1], [2,0], [2,1], [2,1]], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 5, shape : [[0,2], [1,2], [2,2], [2,1], [2,0], [2,0]], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 6, shape : [[0,0], [1,0], [0,1], [1,1], [0,2], [1,2]], orientation : 0, direction : 1 , shape_size : 6},
    Piece {id : 7, shape : [[0,0], [0,1], [0,2], [0,3], [1,3], [1,3]], orientation : 0, direction : 1 , shape_size : 5}];
}

fn play(board: Board, pieces: [Piece; 8]) -> bool {

    let mut game_idx : u32  = 0;
    for idx_0 in 0 .. pieces.len() {                
        for idx_1 in 0 .. pieces.len() {            
            if idx_1 == idx_0 {
                continue;
            }
            for idx_2 in 0 .. pieces.len() {
                if idx_2 == idx_1 || idx_2 == idx_0 {
                    continue;
                }
                for idx_3 in 0 .. pieces.len() {
                    if idx_3 == idx_2 || idx_3 == idx_1 || idx_3 == idx_0 {
                        continue;
                    }
                    for idx_4 in 0 .. pieces.len() {
                        if idx_4 == idx_3 || idx_4 == idx_2 || idx_4 == idx_1 || idx_4 == idx_0 {
                            continue;
                        }
                        for idx_5 in 0 .. pieces.len() {
                            if idx_5 == idx_4 || idx_5 == idx_3 || idx_5 == idx_2 || idx_5 == idx_1 || idx_5 == idx_0 {
                                continue;
                            }
                            for idx_6 in 0 .. pieces.len() {
                                if idx_6 == idx_5 || idx_6 == idx_4 || idx_6 == idx_3 || idx_6 == idx_2 || idx_6 == idx_1 || idx_6 == idx_0 {
                                    continue;
                                }    
                                for idx_7 in 0 .. pieces.len() {
                                    if idx_7 == idx_6 || idx_7 == idx_5 || idx_7 == idx_4 || idx_7 == idx_3 || idx_7 == idx_2 || idx_7 == idx_1 || idx_7 == idx_0 {
                                        continue;
                                    }
                                    println!("{} : {} {} {} {} {} {} {} {}", game_idx, idx_0, idx_1, idx_2, idx_3,idx_4, idx_5,idx_6, idx_7);
                                    game_idx = game_idx + 1;
                                    let game_pieces : [Piece; 8] = [
                                        pieces[idx_0],
                                        pieces[idx_1],
                                        pieces[idx_2],
                                        pieces[idx_3],
                                        pieces[idx_4],
                                        pieces[idx_5],
                                        pieces[idx_6],
                                        pieces[idx_7]
                                    ];

                                    let mut game : Game = Game{board : board.clone(), pieces : game_pieces, id : game_idx, complete : false};                                    
                                    game.play();
                                    if game.complete {
                                        println!("{} : complete", game_idx);
                                        game.draw();
                                        return true; 
                                    }                                
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return false;
}



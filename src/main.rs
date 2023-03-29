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
    println!("Pieces");
    for idx in 0 .. pieces.len() {
        //println!("{:?}", pieces[idx]);
        pieces[idx].draw()
    }
    println!("Start");
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
        let mut piece_0 = pieces[idx_0];
        for _r_0 in 0 ..=3 {
            for _f_0 in 0 ..=1 {
                for idx_1 in 0 .. pieces.len() {
                    if idx_1 == idx_0 {
                        continue;
                    }
                    let mut piece_1 = pieces[idx_1];
                    for _r_1 in 0 ..=3 {
                        for _f_1 in 0 ..=1 {            
                            for idx_2 in 0 .. pieces.len() {
                                if idx_2 == idx_1 || idx_2 == idx_0 {
                                    continue;
                                }
                                let mut piece_2 = pieces[idx_2];
                                for _r_2 in 0 ..=3 {
                                    for _f_2 in 0 ..=1 {            
            
                                        for idx_3 in 0 .. pieces.len() {
                                            if [idx_2, idx_1, idx_0].contains(&idx_3) {
                                                continue;
                                            }
                                            let mut piece_3 = pieces[idx_3];
                                            for _r_3 in 0 ..=3 {
                                                for _f_3 in 0 ..=1 {      
                                                    for idx_4 in 0 .. pieces.len() {
                                                        if [idx_3, idx_2, idx_1, idx_0].contains(&idx_4) {
                                                            continue;
                                                        }
                                                        let mut piece_4 = pieces[idx_4];
                                                        for _r_4 in 0 ..=3 {
                                                            for _f_4 in 0 ..=1 {      
            
                                                                for idx_5 in 0 .. pieces.len() {
                                                                    if [idx_4, idx_3, idx_2, idx_1, idx_0].contains(&idx_5) {
                                                                        continue;
                                                                    }
                                                                    let mut piece_5 = pieces[idx_5];
                                                                    for _r_5 in 0 ..=3 {
                                                                        for _f_5 in 0 ..=1 {
            
                                                                            for idx_6 in 0 .. pieces.len() {
                                                                                if [idx_5, idx_4, idx_3, idx_2, idx_1, idx_0].contains(&idx_6) {
                                                                                    continue;
                                                                                }
                                                                                let mut piece_6 = pieces[idx_6];
                                                                                for _r_6 in 0 ..=3 {
                                                                                    for _f_6 in 0 ..=1 {     
                                                                                        for idx_7 in 0 .. pieces.len() {
                                                                                            if [idx_6, idx_5, idx_4, idx_3, idx_2, idx_1, idx_0].contains(&idx_7) {                                    
                                                                                                continue;
                                                                                            }
                                                                                            let mut piece_7 = pieces[idx_7];
                                                                                            for _r_7 in 0 ..=3 {
                                                                                                for _f_7 in 0 ..=1 {     
            
                                                                                                    
                                                                                                    game_idx = game_idx + 1;
                                                                                                    let game_pieces : [Piece; 8] = [
                                                                                                        piece_0,
                                                                                                        piece_1,
                                                                                                        piece_2,
                                                                                                        piece_3,
                                                                                                        piece_4,
                                                                                                        piece_5,
                                                                                                        piece_6,
                                                                                                        piece_7
                                                                                                    ];

                                                                                                    let mut game : Game = Game{board : board.clone(), pieces : game_pieces, id : game_idx, complete : false};                                    

                                                                                                    game.play();
                                                                                                    if game.complete {
                                                                                                        println!("{} : complete", game_idx);
                                                                                                        game.draw();
                                                                                                        return true; 
                                                                                                    }
                                                                                                    if game_idx % 10000 == 0 {
                                                                                                        println!("{} : {} {} {} {} {} {} {} {}", game_idx, idx_0, idx_1, idx_2, idx_3,idx_4, idx_5,idx_6, idx_7);
                                                                                                        game.draw();
                                                                                                    }

                                                                                                    piece_7 = piece_7.flip();            
                                                                                                }
                                                                                                piece_7 = piece_7.rotate();
                                                                                            }
                                                                                        }
                                                                                        piece_6 = piece_6.flip();
                                                                                    }
                                                                                    piece_6 = piece_6.rotate();
                                                                                }
                                                                            }
                                                                            piece_5 = piece_5.flip();
                                                                        }                                                                
                                                                        piece_5 = piece_5.rotate();
                                                                    }
                                                                }
                                                                piece_4 = piece_4.flip();
                                                            }
                                                            piece_4 = piece_4.rotate();
                                                        }
                                                    }
                                                    piece_3 = piece_3.flip();
                                                }
                                                piece_3 = piece_3.rotate();
                                            }
                                        }
                                        piece_2 = piece_2.flip();
                                    }
                                    piece_2 = piece_2.rotate();
                                }
                            }
                            piece_1 = piece_1.flip();
                        }
                        piece_1 = piece_1.rotate();
                    }
                }
                piece_0 = piece_0.flip();    
            }
            piece_0 = piece_0.rotate();
        }
    }
    println!("{} : iterations", game_idx);
    return false;
}



use std::vec;

mod piece;
mod game;
mod tile;
mod board;

use crate::tile::Tile;
use crate::game::Game;
use crate::piece::Piece;
use crate::board::Board;

fn main() {
    
    // Create the board
    let mut b = Board::new();
    b.init();
    
    b.reserve(String::from("Feb"));
    b.reserve(String::from("20"));

    b.draw();

    // create the pieces    
    let pieces = create_pieces(); 

    for p in &pieces[..] {
        println!("{:?}", p);
        p.draw()
    }

    play(b, pieces);
}

fn create_pieces() -> Vec<Piece> {
    let mut pieces : Vec<Piece> = Vec::new();
    pieces.push(Piece {id : 0, shape : vec![[0,0], [0,1], [0,2], [0,3], [1,1] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 1, shape : vec![[0,0], [0,1], [1,0], [2,0], [2,1] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 2, shape : vec![[0,0], [0,1], [1,1], [2,1], [2,2] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 3, shape : vec![[1,0], [1,1], [1,2], [0,2], [0,3] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 4, shape : vec![[1,0], [0,1], [1,1], [2,0], [2,1] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 5, shape : vec![[0,2], [1,2], [2,2], [2,1], [2,0] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 6, shape : vec![[0,0], [1,0], [0,1], [1,1], [0,2], [1,2] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 7, shape : vec![[0,0], [0,1], [0,2], [0,3], [1,3] ], orientation : 0, direction : 1 });
    return pieces;
}

fn play(board: Board, pieces: Vec<Piece>) -> bool {
    let mut games :Vec<&Game> = vec![];
    let mut game : Game = Game{board : board.clone(), pieces : vec![], id : 1, complete : false};
    games.push(&game);
    // let's start
    // outer loop starts with each piece
    'pLoop: for p in pieces {
        // rotation loop
        for _r in 0 ..=3 {
            let rotated = p.rotate();
            // flip loop
            for _d in 0 .. 2 {
                let flipped = rotated.flip();
                for i in 0 .. game.board.len() {

                    if game.board.spaces[i].used != 0 {
                        continue;
                    }
                    if ! can_place(&game.board.spaces, &game.board.spaces[i], &flipped) {
                        continue
                    }

                    // place
                    println!("placing {} @ {},{}", flipped.id, game.board.spaces[i].x, game.board.spaces[i].y );
                    for c in &flipped.shape[..] {
                        //println!("shape {} @ {},{}", flipped.id, c[0], c[1] );
                        for ii in 0 .. game.board.len() {          
                            //println!("checking {} @ {},{}", flipped.id, game.board[ii].x, game.board[ii].y ); 
                            //println!("against  {} @ {},{}", flipped.id, game.board[i].x + c[0], game.board[i].y+c[1] );                 
                            if game.board.spaces[ii].x == game.board.spaces[i].x + c[0] && game.board.spaces[ii].y == game.board.spaces[i].y + c[1] && game.board.spaces[ii].used == 0 {
                                //println!("using {} @ {},{}", flipped.id, game.board[ii].x, game.board[ii].y );
                                game.board.spaces[ii].used(p.id);
                            }            
                        }
                    }
                    game.board.draw();
                    continue 'pLoop;
                }            
            }
        }
    }
    return false
}

fn can_place(board : &Vec<Tile>, t: &Tile, p: &Piece) -> bool {
    // in bounds
    for c in &p.shape[..] {
        let mut found = false;
        for tt in &board[..] {
            if tt.x == t.x + c[0] && tt.y == t.y + c[1] {
                found = true;
                break
            }                    
        }
        if !found {
            return false;
        }
    }
    
    // check collision
    for c in &p.shape[..] {
        for tt in &board[..] {
            if tt.x == t.x + c[0] && tt.y == t.y + c[1] && tt.used != 0 {
                return false
            }            
        }
    }

    return true;
}

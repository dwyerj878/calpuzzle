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
    pieces.push(Piece {id : 0, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1]], orientation : 0, direction : 1 , shape_size : 5});
    pieces.push(Piece {id : 1, shape : [[0,0], [0,1], [1,0], [2,0], [2,1], [2,1]], orientation : 0, direction : 1 , shape_size : 5});
    pieces.push(Piece {id : 2, shape : [[0,0], [0,1], [1,1], [2,1], [2,2], [2,2]], orientation : 0, direction : 1 , shape_size : 5});
    pieces.push(Piece {id : 3, shape : [[1,0], [1,1], [1,2], [0,2], [0,3], [0,3]], orientation : 0, direction : 1 , shape_size : 5});
    pieces.push(Piece {id : 4, shape : [[1,0], [0,1], [1,1], [2,0], [2,1], [2,1]], orientation : 0, direction : 1 , shape_size : 5});
    pieces.push(Piece {id : 5, shape : [[0,2], [1,2], [2,2], [2,1], [2,0], [2,0]], orientation : 0, direction : 1 , shape_size : 5});
    pieces.push(Piece {id : 6, shape : [[0,0], [1,0], [0,1], [1,1], [0,2], [1,2]], orientation : 0, direction : 1 , shape_size : 6});
    pieces.push(Piece {id : 7, shape : [[0,0], [0,1], [0,2], [0,3], [1,3], [1,3]], orientation : 0, direction : 1 , shape_size : 5});
    return pieces;
}

fn play(board: Board, pieces: Vec<Piece>) -> bool {
    let games :Vec<&Game> = vec![];
    let mut game : Game = Game{board : board.clone(), pieces : pieces, id : 1, complete : false};
    game.play(games);
    game.draw(); 

    return false
}



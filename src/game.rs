
#[path="piece.rs"]
mod p;

#[path="tile.rs"]
mod t;


pub struct Game {
    pub id : i8,
    pub available_pieces : Vec<p::Piece>,
    pub board : Vec<t::Tile>,
    pub complete : bool
}



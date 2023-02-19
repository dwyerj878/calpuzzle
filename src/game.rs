
use crate::tile::Tile;
use crate::piece::Piece;

pub struct Game {
    pub id : i8,
    pub pieces : Vec<Piece>,
    pub board : Vec<Tile>,
    pub complete : bool
}

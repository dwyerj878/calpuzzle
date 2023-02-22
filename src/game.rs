
use crate::piece::Piece;
use crate::board::Board;


#[derive(Debug, Clone)]
pub struct Game {
    pub id : i8,
    pub pieces : Vec<Piece>,
    pub board : Board,
    pub complete : bool
}


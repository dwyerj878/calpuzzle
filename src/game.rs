
use crate::piece::Piece;
use crate::board::Board;


#[derive(Debug, Clone)]
pub struct Game {
    pub id : i8,
    pub pieces : Vec<Piece>,
    pub board : Board,
    pub complete : bool
}

impl Game {
    pub fn can_place(&self, x: i8, y: i8, p: &Piece) -> bool {
        // in bounds
        for c in &p.shape[..] {
            let mut found = false;
            for tt in self.board.spaces.clone() {
                if tt.x == x + c[0] && tt.y == y + c[1] {
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
            for tt in self.board.spaces.clone() {
                if tt.x == x + c[0] && tt.y == y + c[1] && tt.used != 0 {
                    return false
                }            
            }
        }
    
        return true;
    }
}

#[test]
fn test_placement_inside_boundary () {
    let mut b = Board::new();
    
    let bx = b.init();
    
    let mut pieces : Vec<Piece> = Vec::new();
    pieces.push(Piece {id : 0, shape : vec![[0,0], [0,1], [0,2], [0,3], [1,1] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 1, shape : vec![[0,0], [0,1], [1,0], [2,0], [2,1] ], orientation : 0, direction : 1 });
    
    
    let g: Game = Game{board : bx.clone(), complete: false, id : 1, pieces : pieces.clone()};
    let bb = bx.clone();

    let pp = pieces.clone();
    let first = g.can_place(bb.spaces[1].x, bb.spaces[1].y  , &pp[0]);    
    assert!(first );
}

#[test]
fn test_placement_outside_boundary () {
    let mut b = Board::new();
    
    let bx = b.init();
    
    let mut pieces : Vec<Piece> = Vec::new();
    pieces.push(Piece {id : 0, shape : vec![[0,0], [0,1], [0,2], [0,3], [1,1] ], orientation : 0, direction : 1 });
    pieces.push(Piece {id : 1, shape : vec![[0,0], [0,1], [1,0], [2,0], [2,1] ], orientation : 0, direction : 1 });
    
    
    let g: Game = Game{board : bx.clone(), complete: false, id : 1, pieces : pieces.clone()};
    let bb = bx.clone();

    let pp = pieces.clone();

    let first = g.can_place(bb.spaces[5].x, bb.spaces[5].y  , &pp[1]);

    assert!(!first);
}



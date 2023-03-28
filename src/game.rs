
use crate::piece::Piece;
use crate::board::Board;
use colored::Colorize;

#[derive(Debug)]
pub struct Game {
    pub id : u32,    
    pub board : Board,
    pub complete : bool,
    pub pieces : [Piece; 8]
}

impl Game {
    /**
     * test if we can place a Piece at a particular location
     */
    pub fn can_place(&self, i: usize, p: &Piece) -> bool {
        let x = self.board.spaces[i].x;
        let y = self.board.spaces[i].y;
        // in bounds
        for c_idx in 0 .. p.shape_size {        
            let mut found = false;
            for idx in 0 .. self.board.spaces.len() {            
                if self.board.spaces[idx].x == x + &p.shape[c_idx][0] && self.board.spaces[idx].y == y + &p.shape[c_idx][1] {
                    found = true;
                    break;
                }                    
            }
            if !found {
                return false;
            }
        }
        
        // check collision
        for c_idx in 0 .. p.shape_size { 
            for idx in 0 .. self.board.spaces.len() {
                if self.board.spaces[idx].x == x + &p.shape[c_idx][0] && self.board.spaces[idx].y == y + &p.shape[c_idx][1] && self.board.spaces[idx].used != 0 {
                    return false
                }            
            }
        }
        return true;
    }

    pub fn draw(&self) {        
        let utxt:String = format!("Game : {:>5}", self.id);
        println!("{}", utxt.white().bold());        
        self.board.draw();
    }

    pub fn is_complete(&self) -> bool {
        for p_idx in 0 .. self.pieces.len() {
            let mut p_found : bool = false;
            for t_idx in 0 .. self.board.len() {
                if self.board.spaces[t_idx].used as i8 == p_idx as i8 {
                    p_found = true;
                    break;
                }
            }
            if !p_found {
                return false;
            }
        }
        return false;
    }

    pub fn play(&mut self) {
        // let's start
        // outer loop starts with each piece
        'pLoop: 
        for p_idx in 0..self.pieces.len() {
        // for p in &self.pieces {
            let p = &self.pieces[p_idx];
            // rotation loop
            for _r in 0 ..=3 {
                let rotated = p.rotate();
                // flip loop
                for _d in 0 .. 2 {
                    let flipped = rotated.flip();
                    for i in 0 .. self.board.len() {
                            
                        let space = &self.board.spaces[i];

                        if space.used != 0 {
                            continue;
                        }
                        if ! self.can_place(i, &flipped) {
                            continue
                        }

                        // place
                        self.place(&flipped, i);
                    
                        
                        //self.draw();
                        // self.pieces.remove(index);
                        // TODO = Remove from available pieces
                        continue 'pLoop;
                    }            
                }
            }
        }
        self.complete = self.is_complete();
    }

    fn place(&mut self, flipped: &Piece, i: usize) {
        //println!("placing {} @ {},{}", flipped.id, self.board.spaces[i].x, self.board.spaces[i].y );
        for c in &flipped.shape[..] {
            for ii in 0 .. self.board.len() {          
                if self.board.spaces[ii].x == self.board.spaces[i].x + c[0] && self.board.spaces[ii].y == self.board.spaces[i].y + c[1] && self.board.spaces[ii].used == 0 {
                    self.board.spaces[ii].used(flipped.id);
                }            
            }
        }
    }
}

#[test]
/**
 * ensure that when we try and place a piece that does not fit on the board we fail
 */
fn test_placement_inside_boundary () {
    let b = Board::new();
    
    let pieces = [
    Piece {id : 0, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 1, shape : [[0,0], [0,1], [1,0], [2,0], [2,1] , [2,2]], orientation : 0, direction : 1 , shape_size : 5},
    Piece {id : 2, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 3, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 4, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 5, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 6, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 7, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 }];
    
    let g: Game = Game{board : b.clone(), complete: false, id : 1, pieces : pieces};

    let can_place = g.can_place(1, &pieces[0]);    
    assert!(can_place );
}

#[test]
/**
 * ensure that when we try and place a piece that fitson the board we succeed
 */
fn test_placement_outside_boundary () {
    let b = Board::new();
    
    let pieces = [
    Piece {id : 0, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 1, shape : [[0,0], [0,1], [1,0], [2,0], [2,1] , [2,2]], orientation : 0, direction : 1 , shape_size : 5 },
    Piece {id : 2, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 3, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 4, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 5, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 6, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 7, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 }];
    
    let g: Game = Game{board : b.clone(), complete: false, id : 1, pieces : pieces.clone()};

    let can_place = g.can_place(5, &pieces[1]);

    assert!(!can_place);
}

#[test]
/**
 * ensure that when we try and place a piece that fitson the board we succeed
 */
fn test_placement_overlapping () {
    let b = Board::new();
    
    let pieces = [
    Piece {id : 0, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 1, shape : [[0,0], [0,1], [1,0], [2,0], [2,1] , [2,2]], orientation : 0, direction : 1 , shape_size : 5 },
    Piece {id : 2, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 3, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 4, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 5, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 6, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 },
    Piece {id : 7, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 }];
    
    let mut g: Game = Game{board : b.clone(), complete: false, id : 1, pieces : pieces.clone()};


    g.place(&pieces[1], 1);
    let can_place = g.can_place(1,&pieces[1]);

    assert!(!can_place);
}


use crate::piece::Piece;
use crate::board::Board;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Game {
    pub id : i8,
    pub pieces : Vec<Piece>,
    pub board : Board,
    pub complete : bool
}

impl Game {
    /**
     * test if we can place a Piece at a particular location
     */
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

    pub fn draw(&self) {        
        let utxt:String = format!("Game : {:>5}", self.id);
        println!("{}", utxt.white().bold());        
        self.board.draw();
    }

    pub fn piece(&self, idx : usize) -> Piece {
         Piece { id: self.pieces[idx].id, 
            shape: self.pieces[idx].shape.clone(), 
            orientation: self.pieces[idx].orientation, 
            direction: self.pieces[idx].direction,  
            shape_size: self.pieces[idx].shape_size}
    }

    pub fn play(&mut self, games :Vec<&Game> ) {
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
                            
                        let space = &self.board.spaces[i].clone();

                        if space.used != 0 {
                            continue;
                        }
                        if ! self.can_place(space.x, space.y, &flipped) {
                            continue
                        }

                        // place
                        self.place(&flipped, i, self.piece(p_idx));
                    
                        
                        self.draw();
                        // self.pieces.remove(index);
                        // TODO = Remove from available pieces
                        continue 'pLoop;
                    }            
                }
            }
        }
    }

    fn place(&mut self, flipped: &Piece, i: usize, p: Piece) {
        println!("placing {} @ {},{}", flipped.id, self.board.spaces[i].x, self.board.spaces[i].y );
        for c in &flipped.shape[..] {
            //println!("shape {} @ {},{}", flipped.id, c[0], c[1] );
            for ii in 0 .. self.board.len() {          
                //println!("checking {} @ {},{}", flipped.id, game.board[ii].x, game.board[ii].y ); 
                //println!("against  {} @ {},{}", flipped.id, game.board[i].x + c[0], game.board[i].y+c[1] );                 
                if self.board.spaces[ii].x == self.board.spaces[i].x + c[0] && self.board.spaces[ii].y == self.board.spaces[i].y + c[1] && self.board.spaces[ii].used == 0 {
                    //println!("using {} @ {},{}", flipped.id, game.board[ii].x, game.board[ii].y );
                    self.board.spaces[ii].used(p.id);
                }            
            }
        }
    }
}

#[test]
fn test_placement_inside_boundary () {
    let b = Board::new();
    

    
    let mut pieces : Vec<Piece> = Vec::new();
    pieces.push(Piece {id : 0, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1 , shape_size : 5});
    pieces.push(Piece {id : 1, shape : [[0,0], [0,1], [1,0], [2,0], [2,1] , [2,2]], orientation : 0, direction : 1 , shape_size : 5});
    
    
    let g: Game = Game{board : b.clone(), complete: false, id : 1, pieces : pieces.clone()};
    let bb = b.clone();

    let pp = pieces.clone();
    let first = g.can_place(bb.spaces[1].x, bb.spaces[1].y  , &pp[0]);    
    assert!(first );
}

#[test]
fn test_placement_outside_boundary () {
    let b = Board::new();
    
    let mut pieces : Vec<Piece> = Vec::new();
    pieces.push(Piece {id : 0, shape : [[0,0], [0,1], [0,2], [0,3], [1,1], [1,1] ], orientation : 0, direction : 1, shape_size : 5 });
    pieces.push(Piece {id : 1, shape : [[0,0], [0,1], [1,0], [2,0], [2,1] , [2,2]], orientation : 0, direction : 1 , shape_size : 5 });
    
    
    let g: Game = Game{board : b.clone(), complete: false, id : 1, pieces : pieces.clone()};
    let bb = b.clone();

    let pp = pieces.clone();

    let first = g.can_place(bb.spaces[5].x, bb.spaces[5].y  , &pp[1]);

    assert!(!first);
}



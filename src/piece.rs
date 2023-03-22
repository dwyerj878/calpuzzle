use std::{usize};

#[derive(Debug, Clone)]
pub struct Piece {
    pub id : i8,    
    pub orientation : i8,
    pub direction : i8,
    pub shape_size : usize,
    pub shape : [[i8;2]; 6]    
}


impl Default for Piece {
    fn default() -> Self { 
        Piece {
            id : 0,
            shape : [[0,0], [0,0], [0,0], [0,0], [0,0], [0,0]], 
            orientation : 0, 
            direction : 1,
            shape_size : 5
        }
    }
}


impl Piece {
    /**
    * rotate function uses a transform vector to rotate the shape by 90 degrees
    */
    pub fn rotate(&self) -> Piece {
        println!("stage 0 {:?}", self);
        let mut rotated : Piece = Piece{id: self.id, orientation : self.orientation + 1, direction : self.direction, shape_size : self.shape_size, .. Default::default()};

        let source_v: [[i8;2]; 25] = [[0,0],[0,1],[0,2],[0,3],[0,4],[1,0],[1,1],[1,2],[1,3],[1,4],[2,0],[2,1],[2,2],[2,3],[2,4],[3,0],[3,1],[3,2],[3,3],[3,4],[4,0],[4,1],[4,2],[4,3],[4,4]];
        let transf_v: [[i8;2]; 25] = [[4,0],[3,0],[2,0],[1,0],[0,0],[4,1],[3,1],[2,1],[1,1],[0,1],[4,2],[3,2],[2,2],[1,0],[0,2],[4,3],[3,3],[2,3],[1,0],[0,3],[4,4],[3,4],[2,4],[1,4],[0,4]];

        for idx in 0 .. self.shape_size {
            for sv_idx in 0 .. source_v.len() {
                if source_v[sv_idx][0] == self.shape[idx][0] && source_v[sv_idx][1] == self.shape[idx][1] {
                    rotated.shape[idx] = [transf_v[sv_idx][0], transf_v[sv_idx][1]];
                    break;
                }
            }
        }

        // find minimum values        
        let (min_x, min_y) = rotated.min_offset();

        // relocate to source [0,0]
        if min_x > 0 || min_y > 0 {
            
            for idx in 0 .. rotated.shape_size {
                rotated.shape[idx] = [rotated.shape[idx][0] - min_x, rotated.shape[idx][1] - min_y];
            }
            
        }
        println!("stage 2 {:?}", rotated);
        return rotated;
    }

    /**
     * get the minimum offset of the shape in this piece
     */
    pub fn min_offset(&self) -> (i8, i8) {
        let mut min_x = 4;
        let mut min_y: i8 = 4;

        for idx in 0 .. self.shape_size {
            if self.shape[idx][0] < min_x {
                min_x = self.shape[idx][0];
            }
            if self.shape[idx][1] < min_y {
                min_y = self.shape[idx][1];
            }
        }
        return (min_x, min_y);
    }

    /**
     * get the minimum offset of the shape in this piece
     */
    pub fn max_offset(&self) -> (i8, i8) {
        let mut max_x = 0;
        let mut max_y: i8 = 0;

        for idx in 0 .. self.shape_size {
            if self.shape[idx][0] > max_x {
                max_x = self.shape[idx][0];
            }
            if self.shape[idx][1] > max_y {
                max_y = self.shape[idx][1];
            }
        }
        return (max_x, max_y);
    }


    /**
     * flip a piece around the vertical axis
     */
    pub fn flip(&self) -> Piece{
        let mut flipped : Piece = Piece{id: self.id, orientation : self.orientation, direction : self.direction * -1, shape_size : self.shape_size, .. Default::default()} ;
        let (max_x, _max_y) = self.max_offset();  
        
        for idx in 0 .. self.shape_size {                
            flipped.shape[idx] = [self.shape[idx][0]*-1+max_x, self.shape[idx][1]];
        }
    
        return flipped;
    }

    /**
     * draw this piece
     */
    pub fn draw(&self) {
        let (_max_x, max_y) = self.max_offset();

        let mut row: i8 = 0;
    
        while row <= max_y {
            let mut line = String::from("             ");
            let piece_char  = &self.id.to_string()[..];
            for idx in 0 .. self.shape_size {  
                if self.shape[idx][1] == row {
                    let upos:u8 = u8::try_from(self.shape[idx][0]).unwrap();
                    let pos:usize = usize::from(upos);
                    
                    line.replace_range(pos..pos+1, piece_char);
                }
            }
            println!("{}", line);
            row+=1;    
        }
    }

}







#[test]
fn test_flip() {
    let p = Piece {id : 0, shape : [[0,0], [0,1], [1,1], [1,2], [1,2] , [0,0]] , orientation : 0, direction : 1 , shape_size : 5}; 
    let f = p.flip();
    assert_eq!(p.id, f.id);
    assert_eq!(f.orientation, 0);
    assert_eq!(f.direction, -1);
    assert_eq!(f.shape.len(), 6);
    assert_eq!(f.shape_size, p.shape_size);
    println!("{:?}", f);    
    assert!(f.shape.contains(&[1,0]));
    assert!(f.shape.contains(&[1,1]));
    assert!(f.shape.contains(&[0,1]));
    assert!(f.shape.contains(&[0,2]));
}

#[test]
fn test_rotate() {
    let p = Piece {id : 0, shape : [[0,0], [0,1], [1,1], [1,2], [0,0], [0,0] ], orientation : 0, direction : 1 , shape_size : 4}; 
    let r = p.rotate();
    assert_eq!(p.id, r.id);
    assert_eq!(r.orientation, 1);
    assert_eq!(r.direction, 1);
    assert_eq!(r.shape.len(), 6);
    assert_eq!(r.shape_size, p.shape_size);
    println!("{:?}", r);
    assert!(r.shape.contains(&[1,0]));
    assert!(r.shape.contains(&[2,0]));
    assert!(r.shape.contains(&[0,1]));
    assert!(r.shape.contains(&[1,1]));

}
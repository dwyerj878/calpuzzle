use std::{usize};

#[derive(Debug, Clone)]
pub struct Piece {
    pub id : i8,
    pub shape : Vec<[i8;2]>,
    pub orientation : i8,
    pub direction : i8
}

impl Piece {
    /**
    * rotate function uses a transform vector to rotate the shape by 90 degrees
    */
    pub fn rotate(&self) -> Piece {
        let mut rotated : Piece = Piece{id: self.id, shape : Vec::new() , orientation : self.orientation + 1, direction : self.direction};

    let source_v: Vec<[i8;2]> = vec![[0,0],[0,1],[0,2],[0,3],[0,4],[1,0],[1,1],[1,2],[1,3],[1,4],[2,0],[2,1],[2,2],[2,3],[2,4],[3,0],[3,1],[3,2],[3,3],[3,4],[4,0],[4,1],[4,2],[4,3],[4,4]];
    let transf_v: Vec<[i8;2]> = vec![[4,0],[3,0],[2,0],[1,0],[0,0],[4,1],[3,1],[2,1],[1,1],[0,1],[4,2],[3,2],[2,2],[1,0],[0,2],[4,3],[3,3],[2,3],[1,0],[0,3],[4,4],[3,4],[2,4],[1,4],[0,4]];

    
    for p in &self.shape[..] {
        
        for (idx, s) in source_v.iter().enumerate() {            
            if s[0] == p[0] && s[1] == p[1] {
                let transformed = transf_v.get(idx);                
                if transformed.is_none() {
                    println!("oops!!");
                } else {
                    let t = *transformed.unwrap();
                    rotated.shape.push([t[0], t[1]]);
                    break;
                } 
            }
        }
    }

    // find minimum values
    let mut min_x = 4;
    let mut min_y: i8 = 4;

    for p in &rotated.shape[..] {
        if p[0] < min_x {
            min_x = p[0];
        }
        if p[1] < min_y {
            min_y = p[1];
        }
    }

    // relocate to source [0,0]
    if min_x > 0 || min_y > 0 {
        let mut relocated : Vec<[i8;2]> = Vec::new();

        for p in &rotated.shape[..] {
            relocated.push([p[0] - min_x, p[1] - min_y]);
        }
        rotated.shape = relocated;   
    }

    return rotated;
    }

    pub fn flip(&self) -> Piece{
        let mut flipped : Piece = Piece{id: self.id, shape : Vec::new() , orientation : self.orientation, direction : self.direction * -1} ;
        let mut max_x = 0;
        for p in &self.shape[..] {
            if p[0] > max_x {
                max_x = p[0];
            }
        }
    
        for p in &self.shape[..] {
            flipped.shape.push([p[0]*-1+max_x, p[1]])
        }
    
        return flipped;
    }


    pub fn draw(&self) {
        let mut max_x = 0;
        let mut max_y = 0;
        for p in &self.shape[..] {
            if p[0] > max_x {
                max_x = p[0];
            }
            if p[1] > max_y {
                max_y = p[1];
            }
        }
    
        let mut row: i8 = 0;
    
        while row <= max_y {
            let mut line = String::from("             ");
            let piece_char  = &self.id.to_string()[..];
            for p in &self.shape[..] {
                if p[1] == row {
                    let upos:u8 = u8::try_from(p[0]).unwrap();
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
    let p = Piece {id : 0, shape : vec![[0,0], [0,1], [1,1], [1,2] ], orientation : 0, direction : 1 }; 
    let f = p.flip();
    assert_eq!(p.id, f.id);
    assert_eq!(f.orientation, 0);
    assert_eq!(f.direction, -1);
    assert_eq!(f.shape.len(), 4);
    println!("{:?}", f);    
    assert!(f.shape.contains(&[1,0]));
    assert!(f.shape.contains(&[1,1]));
    assert!(f.shape.contains(&[0,1]));
    assert!(f.shape.contains(&[0,2]));
}

#[test]
fn test_rotate() {
    let p = Piece {id : 0, shape : vec![[0,0], [0,1], [1,1], [1,2] ], orientation : 0, direction : 1 }; 
    let r = p.rotate();
    assert_eq!(p.id, r.id);
    assert_eq!(r.orientation, 1);
    assert_eq!(r.direction, 1);
    assert_eq!(r.shape.len(), 4);
    println!("{:?}", r);
    assert!(r.shape.contains(&[1,0]));
    assert!(r.shape.contains(&[2,0]));
    assert!(r.shape.contains(&[0,1]));
    assert!(r.shape.contains(&[1,1]));

}
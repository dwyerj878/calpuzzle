use std::vec;

#[path="piece.rs"]
mod p;

fn main() {
    
    let mut board : Vec<Tile> = Vec::new();
    let mut pieces : Vec<p::Piece> = Vec::new();
 
    create(&mut board);
    draw(&board);

    pieces.push(p::Piece {id : 0, shape : vec![[0,0], [0,1], [0,2], [0,3], [1,1] ], orientation : 0, direction : 1 }); 
    pieces.push(p::Piece {id : 1, shape : vec![[0,0], [0,1], [1,0], [2,0], [2,1] ], orientation : 0, direction : 1 });     
    pieces.push(p::Piece {id : 2, shape : vec![[0,0], [0,1], [1,1], [2,1], [2,2] ], orientation : 0, direction : 1 }); 
    pieces.push(p::Piece {id : 3, shape : vec![[1,0], [1,1], [1,2], [0,2], [0,3] ], orientation : 0, direction : 1 }); 
    pieces.push(p::Piece {id : 4, shape : vec![[1,0], [0,1], [1,1], [2,0], [2,1] ], orientation : 0, direction : 1 }); 
    pieces.push(p::Piece {id : 5, shape : vec![[0,2], [1,2], [2,2], [2,1], [2,0] ], orientation : 0, direction : 1 }); 
    pieces.push(p::Piece {id : 6, shape : vec![[0,0], [1,0], [0,1], [1,1], [0,2], [1,2] ], orientation : 0, direction : 1 }); 
    pieces.push(p::Piece {id : 7, shape : vec![[0,0], [0,1], [0,2], [0,3], [1,3] ], orientation : 0, direction : 1 }); 

    for p in &pieces[..] {
        println!("{:?}", p);
        p::draw(&p)
    }

    
    play(&board, &pieces);



}

fn play(board: &Vec<Tile>, pieces: &Vec<p::Piece>) -> bool {
    let mut games :Vec<&Game> = vec![];
    let mut game : Game = Game{board : board.clone(), pieces : vec![]};
    games.push(&game);
    // let's start
    // outer loop starts with each piece
    for p in pieces {
        // rotation loop
        for r in 0 ..=3 {
            let rotated = p::rotate(p);
            // flip loop
            for d in 0 .. 2 {
                let flipped = p::flip(&rotated);
                for i in 0 .. game.board.len() {
                    let t = &game.board[i];
                    if t.used == 0 {
                        if ! can_place(&game.board, &t, &flipped) {
                            continue
                        }

                        // place
                        for c in &flipped.shape[..] {
                            for ii in 0 .. game.board.len() {
                                let tt = &game.board[ii];
                                if tt.x == t.x + c[0] && tt.y == t.y + c[1] && tt.used != 0 {
                                    game.board[ii].used(p.id);
                                }            
                            }
                        }
                        
                    }
                }
            }
        }
    }
    return false
}

fn can_place(board : &Vec<Tile>, t: &Tile, p: &p::Piece) -> bool {
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


    


fn draw(board: &Vec<Tile> ) {
    let mut max_x: i8 = 0;
    let mut max_y: i8 = 0;
    for tile in board {
        if tile.x > max_x {
            max_x = tile.x;
        }
        if tile.y > max_y {
            max_y = tile.y;
        }
    }
    println!("board {0} x {1}", max_x + 1, max_y + 1);

    for y in 0 .. max_y + 1 {    
        for x in 0 .. max_x + 1{
            let mut matched:bool = false;
            for tile in board {                
                if tile.x == x && tile.y == y {
                    print!("|{:5}", tile.txt);
                    matched = true;
                    break;
                }                
            }
            if !matched {
                print!("|  -  ");
            }                
        }
        println!("|");
    }

}

fn create(board: &mut Vec<Tile> ) {
    board.push(Tile {x:0, y:0, used:0, txt:String::from("Jan")});
    board.push(Tile {x:1, y:0, used:0, txt:String::from("Feb")});
    board.push(Tile {x:2, y:0, used:0, txt:String::from("Mar")});
    board.push(Tile {x:3, y:0, used:0, txt:String::from("Apr")});
    board.push(Tile {x:4, y:0, used:0, txt:String::from("May")});
    board.push(Tile {x:5, y:0, used:0, txt:String::from("Jun")});

    board.push(Tile {x:0, y:1, used:0, txt:String::from("Jul")});
    board.push(Tile {x:1, y:1, used:0, txt:String::from("Aug")});
    board.push(Tile {x:2, y:1, used:0, txt:String::from("Sep")});
    board.push(Tile {x:3, y:1, used:0, txt:String::from("Oct")});
    board.push(Tile {x:4, y:1, used:0, txt:String::from("Nov")});
    board.push(Tile {x:5, y:1, used:0, txt:String::from("Dec")});

    board.push(Tile {x:0, y:2, used:0, txt:String::from("1")});
    board.push(Tile {x:1, y:2, used:0, txt:String::from("2")});
    board.push(Tile {x:2, y:2, used:0, txt:String::from("3")});
    board.push(Tile {x:3, y:2, used:0, txt:String::from("4")});
    board.push(Tile {x:4, y:2, used:0, txt:String::from("5")});
    board.push(Tile {x:5, y:2, used:0, txt:String::from("6")});
    board.push(Tile {x:6, y:2, used:0, txt:String::from("7")});

    board.push(Tile {x:0, y:3, used:0, txt:String::from("8")});
    board.push(Tile {x:1, y:3, used:0, txt:String::from("9")});
    board.push(Tile {x:2, y:3, used:0, txt:String::from("10")});
    board.push(Tile {x:3, y:3, used:0, txt:String::from("11")});
    board.push(Tile {x:4, y:3, used:0, txt:String::from("12")});
    board.push(Tile {x:5, y:3, used:0, txt:String::from("13")});
    board.push(Tile {x:6, y:3, used:0, txt:String::from("14")});

    board.push(Tile {x:0, y:4, used:0, txt:String::from("15")});
    board.push(Tile {x:1, y:4, used:0, txt:String::from("16")});
    board.push(Tile {x:2, y:4, used:0, txt:String::from("17")});
    board.push(Tile {x:3, y:4, used:0, txt:String::from("18")});
    board.push(Tile {x:4, y:4, used:0, txt:String::from("19")});
    board.push(Tile {x:5, y:4, used:0, txt:String::from("20")});
    board.push(Tile {x:6, y:4, used:0, txt:String::from("21")});

    board.push(Tile {x:0, y:5, used:0, txt:String::from("22")});
    board.push(Tile {x:1, y:5, used:0, txt:String::from("23")});
    board.push(Tile {x:2, y:5, used:0, txt:String::from("24")});
    board.push(Tile {x:3, y:5, used:0, txt:String::from("25")});
    board.push(Tile {x:4, y:5, used:0, txt:String::from("26")});
    board.push(Tile {x:5, y:5, used:0, txt:String::from("27")});
    board.push(Tile {x:6, y:5, used:0, txt:String::from("28")});

    board.push(Tile {x:0, y:6, used:0, txt:String::from("29")});
    board.push(Tile {x:1, y:6, used:0, txt:String::from("30")});
    board.push(Tile {x:2, y:6, used:0, txt:String::from("31")});
}

#[derive(Debug, Clone)]
struct Tile { 
    x : i8,
    y : i8,
    used : i8,
    txt : String

  
}

impl Tile {
    fn used(&mut self, p: i8) {
        self.used = p;
    }
}

struct Game {
    board : Vec<Tile>,
    pieces : Vec<p::Piece>
}

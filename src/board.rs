use std::fmt::Error;

use colored::Colorize;
use crate::tile::Tile;

#[derive(Debug, Clone)]
pub struct Board {
    pub spaces : Vec<Tile>
}

impl Board {

    /**
     * Create empty board
     */
    pub fn new() -> Board {
        Board {
            spaces : Vec::new()
        }
    }

    /**
     * initialize standard board
     * 
     */
    pub fn init(&mut self) -> &mut Board {
        self.spaces.push(Tile {x:0, y:0, used:0, txt:String::from("Jan")});
        self.spaces.push(Tile {x:1, y:0, used:0, txt:String::from("Feb")});
        self.spaces.push(Tile {x:2, y:0, used:0, txt:String::from("Mar")});
        self.spaces.push(Tile {x:3, y:0, used:0, txt:String::from("Apr")});
        self.spaces.push(Tile {x:4, y:0, used:0, txt:String::from("May")});
        self.spaces.push(Tile {x:5, y:0, used:0, txt:String::from("Jun")});
    
        self.spaces.push(Tile {x:0, y:1, used:0, txt:String::from("Jul")});
        self.spaces.push(Tile {x:1, y:1, used:0, txt:String::from("Aug")});
        self.spaces.push(Tile {x:2, y:1, used:0, txt:String::from("Sep")});
        self.spaces.push(Tile {x:3, y:1, used:0, txt:String::from("Oct")});
        self.spaces.push(Tile {x:4, y:1, used:0, txt:String::from("Nov")});
        self.spaces.push(Tile {x:5, y:1, used:0, txt:String::from("Dec")});
    
        self.spaces.push(Tile {x:0, y:2, used:0, txt:String::from("1")});
        self.spaces.push(Tile {x:1, y:2, used:0, txt:String::from("2")});
        self.spaces.push(Tile {x:2, y:2, used:0, txt:String::from("3")});
        self.spaces.push(Tile {x:3, y:2, used:0, txt:String::from("4")});
        self.spaces.push(Tile {x:4, y:2, used:0, txt:String::from("5")});
        self.spaces.push(Tile {x:5, y:2, used:0, txt:String::from("6")});
        self.spaces.push(Tile {x:6, y:2, used:0, txt:String::from("7")});
    
        self.spaces.push(Tile {x:0, y:3, used:0, txt:String::from("8")});
        self.spaces.push(Tile {x:1, y:3, used:0, txt:String::from("9")});
        self.spaces.push(Tile {x:2, y:3, used:0, txt:String::from("10")});
        self.spaces.push(Tile {x:3, y:3, used:0, txt:String::from("11")});
        self.spaces.push(Tile {x:4, y:3, used:0, txt:String::from("12")});
        self.spaces.push(Tile {x:5, y:3, used:0, txt:String::from("13")});
        self.spaces.push(Tile {x:6, y:3, used:0, txt:String::from("14")});
    
        self.spaces.push(Tile {x:0, y:4, used:0, txt:String::from("15")});
        self.spaces.push(Tile {x:1, y:4, used:0, txt:String::from("16")});
        self.spaces.push(Tile {x:2, y:4, used:0, txt:String::from("17")});
        self.spaces.push(Tile {x:3, y:4, used:0, txt:String::from("18")});
        self.spaces.push(Tile {x:4, y:4, used:0, txt:String::from("19")});
        self.spaces.push(Tile {x:5, y:4, used:0, txt:String::from("20")});
        self.spaces.push(Tile {x:6, y:4, used:0, txt:String::from("21")});
    
        self.spaces.push(Tile {x:0, y:5, used:0, txt:String::from("22")});
        self.spaces.push(Tile {x:1, y:5, used:0, txt:String::from("23")});
        self.spaces.push(Tile {x:2, y:5, used:0, txt:String::from("24")});
        self.spaces.push(Tile {x:3, y:5, used:0, txt:String::from("25")});
        self.spaces.push(Tile {x:4, y:5, used:0, txt:String::from("26")});
        self.spaces.push(Tile {x:5, y:5, used:0, txt:String::from("27")});
        self.spaces.push(Tile {x:6, y:5, used:0, txt:String::from("28")});
    
        self.spaces.push(Tile {x:0, y:6, used:0, txt:String::from("29")});
        self.spaces.push(Tile {x:1, y:6, used:0, txt:String::from("30")});
        self.spaces.push(Tile {x:2, y:6, used:0, txt:String::from("31")});
        return self;
    }


    /**
     * draw to stdout
     */
    pub fn draw(&mut self) {
        let mut max_x: i8 = 0;
        let mut max_y: i8 = 0;
        for tile in &self.spaces {
            if tile.x > max_x {
                max_x = tile.x;
            }
            if tile.y > max_y {
                max_y = tile.y;
            }
        }
        println!("spaces {0} x {1}", max_x + 1, max_y + 1);
    
        for y in 0 .. max_y + 1 {    
            for x in 0 .. max_x + 1{
                let mut matched:bool = false;
                for tile in &self.spaces {                
                    if tile.x == x && tile.y == y {
                        if tile.used == 0 {
                            print!("|{:^5}", tile.txt.bright_green().on_green());
                        } else if tile.used == -1 {
                            print!("|{:^5}", tile.txt.blue().on_red());                    
                        } else {
                            let utxt:String = format!("{:^5}", tile.used);
                            print!("|{:^5}", utxt.red().bold().on_white());
                        }
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

    /**
     * return number of squares on the board
     */
    pub fn len(&mut self) -> usize {
        return self.spaces.len()
    }
    /**
     * reserve tile with matching text (set used to -1)
     */
    pub fn reserve(&mut self,  txt : String)  {
        for p in 0..self.spaces.len() {
            let mut tile = self.spaces[p].clone();
            if tile.txt.eq_ignore_ascii_case(&txt) {                
                tile.used(-1);
                self.spaces[p] = tile;
                break;
            } 
        }
    }

}


#[test]
fn test_reserve() {
    let mut b = Board::new();
    b.spaces.push(Tile {x:0, y:0, used:0, txt:String::from("Jan")});
    b.spaces.push(Tile {x:1, y:0, used:0, txt:String::from("Feb")});

    b.reserve(String::from("Feb"));

    assert!(b.spaces[0].txt == String::from("Jan"));
    assert!(b.spaces[0].used == 0);
    
    assert!(b.spaces[1].txt == String::from("Feb"));
    assert!(b.spaces[1].used == -1);
    
    
}

#[test]
fn test_len() {
    let mut b = Board::new();
    b.spaces.push(Tile {x:0, y:0, used:0, txt:String::from("Jan")});
    b.spaces.push(Tile {x:1, y:0, used:0, txt:String::from("Feb")});

    assert!(b.len() == 2);
    
    
}

use colored::Colorize;
use crate::tile::Tile;

/**
 * encapsultes vector of tile representing a game board
 * 
 */

#[derive(Debug, Clone)]
pub struct Board {
    //pub spaces : Vec<Tile>
    pub spaces: [Tile; 43 ]
}

impl Board {

    /**
     * Create empty board
     */
    pub fn new() -> Board {
        Board {
            spaces : [
                Tile {x:0, y:0, used:0, txt:String::from("Jan")},
                Tile {x:1, y:0, used:0, txt:String::from("Feb")},
                Tile {x:2, y:0, used:0, txt:String::from("Mar")},
                Tile {x:3, y:0, used:0, txt:String::from("Apr")},
                Tile {x:4, y:0, used:0, txt:String::from("May")},
                Tile {x:5, y:0, used:0, txt:String::from("Jun")},
            
                Tile {x:0, y:1, used:0, txt:String::from("Jul")},
                Tile {x:1, y:1, used:0, txt:String::from("Aug")},
                Tile {x:2, y:1, used:0, txt:String::from("Sep")},
                Tile {x:3, y:1, used:0, txt:String::from("Oct")},
                Tile {x:4, y:1, used:0, txt:String::from("Nov")},
                Tile {x:5, y:1, used:0, txt:String::from("Dec")},
            
                Tile {x:0, y:2, used:0, txt:String::from("1")},
                Tile {x:1, y:2, used:0, txt:String::from("2")},
                Tile {x:2, y:2, used:0, txt:String::from("3")},
                Tile {x:3, y:2, used:0, txt:String::from("4")},
                Tile {x:4, y:2, used:0, txt:String::from("5")},
                Tile {x:5, y:2, used:0, txt:String::from("6")},
                Tile {x:6, y:2, used:0, txt:String::from("7")},
            
                Tile {x:0, y:3, used:0, txt:String::from("8")},
                Tile {x:1, y:3, used:0, txt:String::from("9")},
                Tile {x:2, y:3, used:0, txt:String::from("10")},
                Tile {x:3, y:3, used:0, txt:String::from("11")},
                Tile {x:4, y:3, used:0, txt:String::from("12")},
                Tile {x:5, y:3, used:0, txt:String::from("13")},
                Tile {x:6, y:3, used:0, txt:String::from("14")},
            
                Tile {x:0, y:4, used:0, txt:String::from("15")},
                Tile {x:1, y:4, used:0, txt:String::from("16")},
                Tile {x:2, y:4, used:0, txt:String::from("17")},
                Tile {x:3, y:4, used:0, txt:String::from("18")},
                Tile {x:4, y:4, used:0, txt:String::from("19")},
                Tile {x:5, y:4, used:0, txt:String::from("20")},
                Tile {x:6, y:4, used:0, txt:String::from("21")},
    
                Tile {x:0, y:5, used:0, txt:String::from("22")},
                Tile {x:1, y:5, used:0, txt:String::from("23")},
                Tile {x:2, y:5, used:0, txt:String::from("24")},
                Tile {x:3, y:5, used:0, txt:String::from("25")},
                Tile {x:4, y:5, used:0, txt:String::from("26")},
                Tile {x:5, y:5, used:0, txt:String::from("27")},
                Tile {x:6, y:5, used:0, txt:String::from("28")},
            
                Tile {x:0, y:6, used:0, txt:String::from("29")},
                Tile {x:1, y:6, used:0, txt:String::from("30")},
                Tile {x:2, y:6, used:0, txt:String::from("31")}
            ]
        }
    }

      /**
     * draw to stdout
     */
    pub fn draw(&self) {
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
    pub fn len(&self) -> usize {
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

    b.reserve(String::from("Feb"));

    assert!(b.spaces[0].txt == String::from("Jan"));
    assert!(b.spaces[0].used == 0);
    
    assert!(b.spaces[1].txt == String::from("Feb"));
    assert!(b.spaces[1].used == -1);
}

#[test]
fn test_len() {
    let mut b = 43;
}

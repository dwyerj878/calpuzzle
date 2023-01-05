use std::fmt;

fn main() {
    
    let mut board : Vec<Tile> = Vec::new();
    board.push(Tile {x:0, y:0, used:false, txt:String::from("Jan")});
    board.push(Tile {x:1, y:0, used:false, txt:String::from("Feb")});
    board.push(Tile {x:2, y:0, used:false, txt:String::from("Mar")});
    board.push(Tile {x:3, y:0, used:false, txt:String::from("Apr")});
    board.push(Tile {x:4, y:0, used:false, txt:String::from("May")});
    board.push(Tile {x:5, y:0, used:false, txt:String::from("Jun")});

    board.push(Tile {x:0, y:1, used:false, txt:String::from("Jul")});
    board.push(Tile {x:1, y:1, used:false, txt:String::from("Aug")});
    board.push(Tile {x:2, y:1, used:false, txt:String::from("Sep")});
    board.push(Tile {x:3, y:1, used:false, txt:String::from("Oct")});
    board.push(Tile {x:4, y:1, used:false, txt:String::from("Nov")});
    board.push(Tile {x:5, y:1, used:false, txt:String::from("Dec")});

    board.push(Tile {x:0, y:2, used:false, txt:String::from("1")});
    board.push(Tile {x:1, y:2, used:false, txt:String::from("2")});
    board.push(Tile {x:2, y:2, used:false, txt:String::from("3")});
    board.push(Tile {x:3, y:2, used:false, txt:String::from("4")});
    board.push(Tile {x:4, y:2, used:false, txt:String::from("5")});
    board.push(Tile {x:5, y:2, used:false, txt:String::from("6")});
    board.push(Tile {x:6, y:2, used:false, txt:String::from("7")});

    board.push(Tile {x:0, y:3, used:false, txt:String::from("8")});
    board.push(Tile {x:1, y:3, used:false, txt:String::from("9")});
    board.push(Tile {x:2, y:3, used:false, txt:String::from("10")});
    board.push(Tile {x:3, y:3, used:false, txt:String::from("11")});
    board.push(Tile {x:4, y:3, used:false, txt:String::from("12")});
    board.push(Tile {x:5, y:3, used:false, txt:String::from("13")});
    board.push(Tile {x:6, y:3, used:false, txt:String::from("14")});

    board.push(Tile {x:0, y:4, used:false, txt:String::from("15")});
    board.push(Tile {x:1, y:4, used:false, txt:String::from("16")});
    board.push(Tile {x:2, y:4, used:false, txt:String::from("17")});
    board.push(Tile {x:3, y:4, used:false, txt:String::from("18")});
    board.push(Tile {x:4, y:4, used:false, txt:String::from("19")});
    board.push(Tile {x:5, y:4, used:false, txt:String::from("20")});
    board.push(Tile {x:6, y:4, used:false, txt:String::from("21")});

    board.push(Tile {x:0, y:5, used:false, txt:String::from("22")});
    board.push(Tile {x:1, y:5, used:false, txt:String::from("23")});
    board.push(Tile {x:2, y:5, used:false, txt:String::from("24")});
    board.push(Tile {x:3, y:5, used:false, txt:String::from("25")});
    board.push(Tile {x:4, y:5, used:false, txt:String::from("26")});
    board.push(Tile {x:5, y:5, used:false, txt:String::from("27")});
    board.push(Tile {x:6, y:5, used:false, txt:String::from("28")});

    board.push(Tile {x:3, y:6, used:false, txt:String::from("29")});
    board.push(Tile {x:1, y:6, used:false, txt:String::from("30")});
    board.push(Tile {x:2, y:6, used:false, txt:String::from("31")});
 

    draw(&board);

}


fn draw(board: &Vec<Tile> ) {
    let mut max_x: u8 = 0;
    let mut max_y: u8 = 0;
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

#[derive(Debug, Clone)]
struct Tile { 
    x : u8,
    y : u8,
    used : bool,
    txt : String
}


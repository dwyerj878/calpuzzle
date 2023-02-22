/**
 * Tile is a space on a board that can be empty, reserved or consumed with a Piece
 */
#[derive(Debug, Clone)]
pub struct Tile { 
    pub x : i8,
    pub y : i8,
    pub used : i8,
    pub txt : String

  
}

impl Tile {
    /**
     * used 
     *  0 - unused
     * -1 - reserved
     * >0 - id of piece occupying this tile
     */
    pub fn used(&mut self, p: i8) {
        println!("Tile.used {}:{} {},{}", self.txt, self.used, self.x, self.y);
        self.used = p;        
    }

}

#[test]
/**
 * test used fn
 */
fn test_used() {
    let mut t: Tile = Tile{x : 1, y: 1, used : 0, txt : String::from("test")};
    assert!(t.used == 0);
    t.used(-1);
    assert!(t.used == -1);
}
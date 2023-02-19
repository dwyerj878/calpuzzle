#[derive(Debug, Clone)]
pub struct Tile { 
    pub x : i8,
    pub y : i8,
    pub used : i8,
    pub txt : String

  
}

impl Tile {
    pub fn used(&mut self, p: i8) {
        println!("Tile.used {}:{} {},{}", self.txt, self.used, self.x, self.y);
        self.used = p;
    }

}
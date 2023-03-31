pub struct Posicion{
    x: i8,
    y: i8,
}

impl Posicion{
    pub fn new(x: i8, y: i8) -> Posicion{
        Posicion{x, y}
    }

    pub fn set_posicion(&mut self, x: i8, y: i8){
        self.x = x;
        self.y = y;
    }

    pub fn get_x(&self) -> i8{
        self.x
    }

    pub fn get_y(&self) -> i8{
        self.y
    }
}
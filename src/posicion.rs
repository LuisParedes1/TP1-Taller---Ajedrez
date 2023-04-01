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

    pub fn coinciden_coordenadas(&self, x_atacante: i8, y_atacante: i8) -> bool{
        self.x == x_atacante && self.y == y_atacante
    }
}
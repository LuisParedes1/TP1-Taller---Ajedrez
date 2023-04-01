//! # Posiciones
//!
//! Este módulo contiene la estructura Posicion y sus métodos.
//!
//! Un struct Posicion representa las coordenadas (x,y) que la pieza ocupa en el tablero.
//!
//! Se toma el extremo superior izquierda como la coordenada (0,0)

pub struct Posicion {
    x: i8,
    y: i8,
}

impl Posicion {
    pub fn new(x: i8, y: i8) -> Posicion {
        Posicion { x, y }
    }

    pub fn set_posicion(&mut self, x: i8, y: i8) {
        self.x = x;
        self.y = y;
    }

    pub fn get_x(&self) -> i8 {
        self.x
    }

    pub fn get_y(&self) -> i8 {
        self.y
    }

    pub fn coinciden_coordenadas(&self, x_atacante: i8, y_atacante: i8) -> bool {
        self.x == x_atacante && self.y == y_atacante
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_crea_posicion_correctamente() {
        let posicion = Posicion::new(1, 2);
        assert_eq!(posicion.get_x(), 1);
        assert_eq!(posicion.get_y(), 2);
    }

    #[test]
    fn test_cambia_posicion_correctamente() {
        let mut posicion = Posicion::new(1, 2);
        posicion.set_posicion(3, 4);
        assert_eq!(posicion.get_x(), 3);
        assert_eq!(posicion.get_y(), 4);
    }

    #[test]
    fn test_al_pasar_dos_posiciones_iguales_coinciden_coordenadas_devuelve_true() {
        let posicion = Posicion::new(1, 2);
        assert_eq!(posicion.coinciden_coordenadas(1, 2), true);
    }
}

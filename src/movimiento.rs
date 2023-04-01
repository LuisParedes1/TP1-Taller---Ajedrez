//! # Movimientos
//! 
//! En este modulo se definen los movimientos de cada piezas
//! 
//! 
//! * Un Rey: Puede moverse en cualquier dirección (vertical, horizontal y diagonal), avanzando siempre una casilla (exceptuando el caso del enroque).
//! * Una Dama: Tambien puede moverse en cualquier dirección (vertical, horizontal y diagonal), avanzando tantas casillas como se desee.
//! * Dos Alfiles: Solo pueden moverse en direcciones diagonales, avanzando tantas casillas como se desee.
//! * Dos Caballos: Se mueve avanzando dos casillas en vertical y una horizontal, o viceversa (simplificando, se mueve en patron de L), siendo capaz de saltar por encima de otras piezas.
//! * Dos Torres: Solo pueden moverse en direcciones verticales y horizontales, avanzando tantas casillas como se desee.
//! * Ocho peones: Puede avanzar una o dos casillas hacia adelante en su primer movimiento, y avanzar solo una en los siguientes. A diferencia de las demas piezas, el peon no puede retroceder, y solo puede capturar piezas que se encuentren a una casilla de distancia en direccion diagonal (hacia adelante). Para este ejercicio, ignoraremos la captura de peon al paso.


mod caballo;
mod diagonal;
mod horizontal;
mod vertical;

use crate::posicion::Posicion;

// Movimiento de las piezas
use caballo::mover_l;
use diagonal::mover_diagonal;
use horizontal::mover_horizontal;
use vertical::mover_vertical;

const UN_PASO: i8 = 1;
const SIN_LIM_PASOS: i8 = 7;

pub fn mover_rey(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_horizontal(posicion_atacante, posicion_receptor, UN_PASO)
        || mover_vertical(posicion_atacante, posicion_receptor, UN_PASO)
        || mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, true, true)
}

pub fn mover_dama(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_horizontal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
        || mover_vertical(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
        || mover_diagonal(
            posicion_atacante,
            posicion_receptor,
            SIN_LIM_PASOS,
            true,
            true,
        )
}

pub fn mover_torre(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_horizontal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
        || mover_vertical(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
}

pub fn mover_alfil(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_diagonal(
        posicion_atacante,
        posicion_receptor,
        SIN_LIM_PASOS,
        true,
        true,
    )
}

pub fn mover_peon(
    posicion_atacante: &Posicion,
    posicion_receptor: &Posicion,
    color: &String,
) -> bool {
    if color == "blanco" {
        mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, true, false)
    } else {
        mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, false, true)
    }
}

pub fn mover_caballo(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_l(posicion_atacante, posicion_receptor)
}


mod tests {

    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_rey_come_vertical() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 3);
        assert_eq!(mover_rey(&posicion_atacante, &posicion_receptor), true);
    }
    
    #[test]
    fn test_rey_come_horizontal() {
        let posicion_atacante = Posicion::new(4, 2);
        let posicion_receptor = Posicion::new(4, 3);
        assert_eq!(mover_rey(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_rey_come_diagonal() {
        let posicion_atacante = Posicion::new(2, 2);
        let posicion_receptor = Posicion::new(3, 3);
        assert_eq!(mover_rey(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_dama_come_vertical() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 3);
        assert_eq!(mover_dama(&posicion_atacante, &posicion_receptor), true);
    }
    
    #[test]
    fn test_dama_come_horizontal() {
        let posicion_atacante = Posicion::new(4, 2);
        let posicion_receptor = Posicion::new(4, 3);
        assert_eq!(mover_dama(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_dama_come_diagonal() {
        let posicion_atacante = Posicion::new(2, 2);
        let posicion_receptor = Posicion::new(0, 0);
        assert_eq!(mover_dama(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_torre_come_vertical() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 3);
        assert_eq!(mover_torre(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_torre_come_horizontal() {
        let posicion_atacante = Posicion::new(4, 2);
        let posicion_receptor = Posicion::new(4, 3);
        assert_eq!(mover_torre(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_alfil_come_diagonal() {
        let posicion_atacante = Posicion::new(2, 2);
        let posicion_receptor = Posicion::new(0, 0);
        assert_eq!(mover_alfil(&posicion_atacante, &posicion_receptor), true);
    }
    
    #[test]
    fn test_peon_blanco_come_diagonal() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(2, 2);
        assert_eq!(mover_peon(&posicion_atacante, &posicion_receptor, &String::from("blanco")), true);
    }
    
    #[test]
    fn test_peon_negro_come_diagonal() {
        let posicion_atacante = Posicion::new(2, 2);
        let posicion_receptor = Posicion::new(3, 3);
        assert_eq!(mover_peon(&posicion_atacante, &posicion_receptor, &String::from("negro")), true);
    }

    #[test]
    fn test_caballo_no_come(){
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(3, 4);
        assert_eq!(mover_caballo(&posicion_atacante, &posicion_receptor), false);
    }

    #[test]
    fn test_caballo_come() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(1, 2);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

}
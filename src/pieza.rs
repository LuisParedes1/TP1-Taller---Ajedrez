//! # Piezas
//!
//! En este modulo me encargo de crear las piezas y definir comportamiento (atacar a otra pieza)
//!
//! En caso de que no se pueda crear correctamente la pieza, devuelvo un None y la funcion que lo llama se hace cargo.

use crate::posicion::Posicion;

use crate::movimiento::{
    mover_alfil, mover_caballo, mover_dama, mover_peon, mover_rey, mover_torre,
};

/// Variantes de las distintas piezas del juego (Peon, Caballo, Alfil, Torre, Dama, Rey)
pub enum Pieza {
    Peon(Posicion, String),
    Caballo(Posicion, String),
    Alfil(Posicion, String),
    Torre(Posicion, String),
    Dama(Posicion, String),
    Rey(Posicion, String),
}

impl Pieza {
    /// Creacion de pieza dada su letra, posicion y color.
    ///
    /// En caso de que la letra no coincide con alguna pieza (Dama, Rey, Peon, Alfi, Caballo, Torre) devuelve un None.
    pub fn new(pieza: char, posicion: Posicion, color: String) -> Option<Pieza> {
        match pieza {
            'R' | 'r' => Some(Pieza::Rey(posicion, color)),
            'D' | 'd' => Some(Pieza::Dama(posicion, color)),
            'A' | 'a' => Some(Pieza::Alfil(posicion, color)),
            'C' | 'c' => Some(Pieza::Caballo(posicion, color)),
            'T' | 't' => Some(Pieza::Torre(posicion, color)),
            'P' | 'p' => Some(Pieza::Peon(posicion, color)),
            _ => None,
        }
    }

    /// Devuelve si la pieza come a su contrincante
    pub fn captura(&self, contrincante: &Pieza) -> bool {
        match self {
            Pieza::Peon(posicion_atacante, color) => {
                mover_peon(posicion_atacante, contrincante.get_posicion(), color)
            }
            Pieza::Caballo(posicion_atacante, _) => {
                mover_caballo(posicion_atacante, contrincante.get_posicion())
            }
            Pieza::Alfil(posicion_atacante, _) => {
                mover_alfil(posicion_atacante, contrincante.get_posicion())
            }
            Pieza::Torre(posicion_atacante, _) => {
                mover_torre(posicion_atacante, contrincante.get_posicion())
            }
            Pieza::Dama(posicion_atacante, _) => {
                mover_dama(posicion_atacante, contrincante.get_posicion())
            }
            Pieza::Rey(posicion_atacante, _) => {
                mover_rey(posicion_atacante, contrincante.get_posicion())
            }
        }
    }

    /// Devuelve la (Posicion)[Posicion] de la pieza.
    pub fn get_posicion(&self) -> &Posicion {
        match self {
            Pieza::Peon(posicion, ..) => posicion,
            Pieza::Caballo(posicion, ..) => posicion,
            Pieza::Alfil(posicion, ..) => posicion,
            Pieza::Torre(posicion, ..) => posicion,
            Pieza::Dama(posicion, ..) => posicion,
            Pieza::Rey(posicion, ..) => posicion,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_devuelve_none_al_crear_pieza_si_no_es_pieza_valida() {
        use crate::posicion::Posicion;

        let pieza = Pieza::new('X', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_none());
    }

    #[test]
    fn test_crea_peon_correctamente() {
        use crate::posicion::Posicion;

        let pieza = Pieza::new('P', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());

        let pieza = Pieza::new('p', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());
    }

    #[test]
    fn test_crea_rey_correctamente() {
        use crate::posicion::Posicion;

        let pieza = Pieza::new('R', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());

        let pieza = Pieza::new('r', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());
    }

    #[test]
    fn test_crea_dama_correctamente() {
        use crate::posicion::Posicion;

        let pieza = Pieza::new('D', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());

        let pieza = Pieza::new('d', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());
    }

    #[test]
    fn test_crea_alfil_correctamente() {
        use crate::posicion::Posicion;

        let pieza = Pieza::new('A', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());

        let pieza = Pieza::new('a', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());
    }

    #[test]
    fn test_crea_caballo_correctamente() {
        use crate::posicion::Posicion;

        let pieza = Pieza::new('C', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());

        let pieza = Pieza::new('c', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());
    }

    #[test]
    fn test_crea_torre_correctamente() {
        use crate::posicion::Posicion;

        let pieza = Pieza::new('T', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());

        let pieza = Pieza::new('t', Posicion::new(1, 1), "blanco".to_string());
        assert!(pieza.is_some());
    }
}

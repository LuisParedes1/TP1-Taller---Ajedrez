//! # Movimiento diagonal (reina, alfil, peon y rey)
//!
//! En este modulo implementa el movimiento diagonal caracteristico del rey, reina, alfil y peon.
//!
//! Dado que hay distintos tipos de movimientos diagonales entonces pido el parametro "se puede mover hacia adelante" y "se puede mover hacia atras"
//! para poder diferenciar entre el movimiento de un peon blanco y negro y el movimiento de una reina, rey o alfil.
//!
//! Si el atacante coincide con la posicion del receptor entonces se captura la pieza y se devuelve true.
//! Caso contrario se devuelve false.

use crate::posicion::Posicion;

pub fn mover_diagonal(
    posicion_atacante: &Posicion,
    posicion_receptor: &Posicion,
    max_pasos: i8,
    puede_mover_arriba: bool,
    puede_mover_abajo: bool,
) -> bool {
    let mut captura_pieza: bool = false;
    let rango;

    if puede_mover_arriba && !puede_mover_abajo {
        // Caso Peon Blanco
        rango = (-max_pasos)..(0)
    } else if !puede_mover_arriba && puede_mover_abajo {
        // Caso Peon Negro
        rango = (0)..(max_pasos + 1)
    } else {
        // Reina, Rey y Alfil
        rango = (-max_pasos)..(max_pasos + 1)
    }

    for i in rango {
        // Diagonal de izquierda a derecha y de arriba hacia abajo
        if posicion_receptor
            .coinciden_coordenadas(posicion_atacante.get_x() + i, posicion_atacante.get_y() + i)
        {
            captura_pieza = true;
            break;
        }
        // Diaogonal de derecha a izquierda y de arriba hacia abajo
        else if posicion_receptor
            .coinciden_coordenadas(posicion_atacante.get_x() - i, posicion_atacante.get_y() + i)
        {
            captura_pieza = true;
            break;
        }
    }
    captura_pieza
}

mod tests {

    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_atacante_no_come_receptor() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(3, 4);
        let max_pasos = 7;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            false
        );
    }

    #[test]
    fn test_atacante_captura_receptor_tres_izq_tres_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(1, 1);
        let max_pasos = 7;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_atacante_captura_receptor_uno_izq_uno_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(2, 2);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_atacante_captura_receptor_tres_izq_tres_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(0, 6);
        let max_pasos = 7;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_atacante_captura_receptor_uno_izq_uno_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(2, 4);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_atacante_captura_receptor_tres_der_tres_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(6, 0);
        let max_pasos = 7;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_atacante_captura_receptor_uno_der_uno_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 2);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_atacante_captura_receptor_tres_der_tres_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(6, 6);
        let max_pasos = 7;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_atacante_captura_receptor_uno_der_uno_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 4);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_peon_blanco_come_pieza_negra() {
        let posicion_atacante = Posicion::new(4, 4);
        let posicion_receptor = Posicion::new(3, 3);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                false
            ),
            true
        );
    }

    #[test]
    fn test_peon_blanco_pegado_der_come_pieza_negra() {
        let posicion_atacante = Posicion::new(7, 5);
        let posicion_receptor = Posicion::new(6, 4);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                true,
                false
            ),
            true
        );
    }

    #[test]
    fn test_peon_negro_come_pieza_blanca() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 4);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                false,
                true
            ),
            true
        );
    }

    #[test]
    fn test_peon_negro_pegado_der_come_pieza_negra() {
        let posicion_atacante = Posicion::new(6, 4);
        let posicion_receptor = Posicion::new(7, 5);
        let max_pasos = 1;

        assert_eq!(
            mover_diagonal(
                &posicion_atacante,
                &posicion_receptor,
                max_pasos,
                false,
                true
            ),
            true
        );
    }
}

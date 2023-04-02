//! Movimiento horizontal (reina, torre y rey)
//!
//! Me muevo horizontalmente en el tablero y si el atacante coincide con la posicion del receptor entonces se captura la pieza y se devuelve true.
//! Caso contrario se devuelve false.

// Por razones de legibilidad de codigo apago esta regla de clippy
#![allow(clippy::if_same_then_else)]

use crate::posicion::Posicion;

/*
    Me desplazo horizontalmente en el tablero y si las coordenadas coinciden entonces la pieza se come
    Devuelvo si la pieza atacante come a la pieza receptor
*/
pub fn mover_horizontal(
    posicion_atacante: &Posicion,
    posicion_receptor: &Posicion,
    cant_pasos: i8,
) -> bool {
    let mut come_pieza: bool = false;

    for i in 1..(cant_pasos + 1) {
        // Mueve hacia derecha
        if posicion_receptor
            .coinciden_coordenadas(posicion_atacante.get_x(), posicion_atacante.get_y() + i)
        {
            come_pieza = true;
            break;
        }
        // Mueve hacia izquierda
        else if posicion_receptor
            .coinciden_coordenadas(posicion_atacante.get_x(), posicion_atacante.get_y() - i)
        {
            come_pieza = true;
            break;
        }
    }
    come_pieza
}

mod tests {

    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_atacante_no_come_receptor() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 3);
        let cant_pasos = 7;

        assert_eq!(
            mover_horizontal(&posicion_atacante, &posicion_receptor, cant_pasos),
            false
        );
    }

    #[test]
    fn test_atacante_come_receptor_cercano() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(2, 3);
        let cant_pasos = 1;

        assert_eq!(
            mover_horizontal(&posicion_atacante, &posicion_receptor, cant_pasos),
            false
        );
    }

    #[test]
    fn test_atacante_come_receptor_lejano() {
        let posicion_atacante = Posicion::new(6, 3);
        let posicion_receptor = Posicion::new(0, 3);
        let cant_pasos = 7;

        assert_eq!(
            mover_horizontal(&posicion_atacante, &posicion_receptor, cant_pasos),
            false
        );
    }
}

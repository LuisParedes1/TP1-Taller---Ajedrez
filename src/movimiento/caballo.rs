//! # Movimiento del caballo
//! 
//! En este modulo implementa el movimiento en L caracteristico del caballo.
//! 
//! Si el moverse el caballo atacante coincide con la posicion del receptor entonces se captura la pieza y se devuelve true. 
//! Caso contrario se devuelve false.


use crate::posicion::Posicion;

pub fn mover_l(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    let mut captura_pieza = false;

    for i in 0..2 {
        // Uno/Dos derecha y Uno/Dos para arriba o para abajo
        if posicion_receptor.coinciden_coordenadas(
            posicion_atacante.get_x() + 2 - i,
            posicion_atacante.get_y() + 1 + i,
        ) || posicion_receptor.coinciden_coordenadas(
            posicion_atacante.get_x() + 2 - i,
            posicion_atacante.get_y() - (1 + i),
        ) {
            captura_pieza = true;
            break;
        }
        // Uno/Dos izquierda y Uno/Dos para arriba o para abajo
        else if posicion_receptor.coinciden_coordenadas(
            posicion_atacante.get_x() - (2 - i),
            posicion_atacante.get_y() + 1 + i,
        ) || posicion_receptor.coinciden_coordenadas(
            posicion_atacante.get_x() - (2 - i),
            posicion_atacante.get_y() - (1 + i),
        ) {
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
    fn test_atacante_no_come_receptor(){
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(3, 4);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), false);
    }

    #[test]
    fn test_atacante_captura_receptor_uno_izq_dos_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(1, 2);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_uno_izq_dos_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(5, 2);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_dos_izq_uno_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(1, 2);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_dos_izq_uno_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(1, 4);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_uno_der_dos_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 1);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_uno_der_dos_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(4, 5);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_dos_der_uno_arriba() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(5, 2);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_dos_der_uno_abajo() {
        let posicion_atacante = Posicion::new(3, 3);
        let posicion_receptor = Posicion::new(5, 4);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    // Caso borde
    #[test]
    fn test_atacante_captura_receptor_en_esquina_inferior_derecha() {
        let posicion_atacante = Posicion::new(7, 7);
        let posicion_receptor = Posicion::new(6, 5);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

    #[test]
    fn test_atacante_captura_receptor_en_esquina_superior_izquierda() {
        let posicion_atacante = Posicion::new(0, 0);
        let posicion_receptor = Posicion::new(1, 2);
        assert_eq!(mover_l(&posicion_atacante, &posicion_receptor), true);
    }

}
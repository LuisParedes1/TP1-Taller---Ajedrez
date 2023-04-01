/*
    Movimiento del caballo
*/

use crate::posicion::Posicion;

pub fn mover_l(posicion_atacante:&Posicion, posicion_receptor: &Posicion) -> bool{

    let mut captura_pieza = false;
    
    for i in 0..2{
        
        // Uno/Dos derecha y Uno/Dos para arriba o para abajo
        if posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x() + 2-i, posicion_atacante.get_y() + 1+i) ||
            posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x() + 2-i, posicion_atacante.get_y() - (1+i)){
                captura_pieza = true;
                break;
        }

        // Uno/Dos izquierda y Uno/Dos para arriba o para abajo
        else if posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x() - (2+i), posicion_atacante.get_y() + 1+i) ||
            posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x() - (2+i), posicion_atacante.get_y() - (1+i)){
                captura_pieza = true;
                break;
        }
    }
    captura_pieza
}
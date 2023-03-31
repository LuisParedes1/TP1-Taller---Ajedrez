/*
    Movimiento del caballo
*/

use crate::posicion::Posicion;

pub fn mover_caballo(posicion_atacante:&Posicion, posicion_receptor: &Posicion) -> bool{

    let mut captura_pieza = false;
    
    for i in 0..2{
        
        // Uno/Dos derecha y Uno/Dos para arriba o para abajo
        if ( coinciden_x(posicion_atacante.get_x() + 2-i , posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() + 1+i , posicion_receptor.get_y())
            ) || 
            ( coinciden_x(posicion_atacante.get_x() + 2-i , posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() - (1+i), posicion_receptor.get_y()) ){
                captura_pieza = true;
        }

        // Uno/Dos izquierda y Uno/Dos para arriba o para abajo
        else if ( coinciden_x(posicion_atacante.get_x() - (2+i) , posicion_receptor.get_x()) && 
                    coinciden_y(posicion_atacante.get_y() + 1+i , posicion_receptor.get_y()) 
                ) || 
            ( coinciden_x(posicion_atacante.get_x() - (2+i) , posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() - (1+i), posicion_receptor.get_y()) ){
                captura_pieza = true;
        }
    }
    captura_pieza
}
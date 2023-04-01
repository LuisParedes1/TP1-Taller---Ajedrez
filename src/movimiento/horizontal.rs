/*
    Movimiento horizontal (reina, torre y rey)
*/

use crate::posicion::Posicion;

/*
    Me desplazo horizontalmente en el tablero y si las coordenadas coinciden entonces la pieza se come
    Devuelvo si la pieza atacante come a la pieza receptor
*/
pub fn mover_horizontal(posicion_atacante:&Posicion, posicion_receptor: &Posicion, cant_pasos:i8)-> bool{

    let mut come_pieza:bool = false;

    for i in 1..(cant_pasos+1){
        
        // Mueve hacia derecha
        if posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x(), posicion_atacante.get_y() + i){
            come_pieza = true;
            break;
        }

        // Mueve hacia izquierda
        else if posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x(), posicion_atacante.get_y() - i){
            come_pieza = true;
            break;
        }
    }    
    come_pieza
}
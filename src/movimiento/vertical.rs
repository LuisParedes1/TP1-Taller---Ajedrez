/*
    Movimiento vertical (reina, torre y rey)
*/

use crate::posicion::Posicion;

/*
    Me desplazo verticalmente en el tablero y si las coordenadas coinciden entonces la pieza se come.
    Devuelvo si la pieza atacante come a la pieza receptor
*/
fn mover_vertical(posicion_atacante:&Posicion, posicion_receptor: &Posicion, cant_pasos:i8)-> bool{

    let mut come_pieza:bool = false;

    for i in 1..(cant_pasos+1){
        
        // arriba
        if coinciden_x(posicion_atacante.get_x() - i as i8, posicion_receptor.get_x()) && 
            coinciden_y(posicion_atacante.get_y(), posicion_receptor.get_y()){
                come_pieza = true;
                break;
        }
        
        // abajo
        else if coinciden_x(posicion_atacante.get_x() + i as i8, posicion_receptor.get_x()) && 
            coinciden_y(posicion_atacante.get_y(), posicion_receptor.get_y()){
                come_pieza = true;
                break;
        }
    }    
    come_pieza
}
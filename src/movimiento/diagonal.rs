/*
    Movimiento diagonal (reina, alfil, peon y rey)
*/

use crate::posicion::Posicion;

pub fn mover_diagonal(posicion_atacante:&Posicion, posicion_receptor: &Posicion, max_pasos:i8, puede_mover_arriba:bool, puede_mover_abajo:bool)-> bool{

    let mut captura_pieza:bool = false;
    let rango;

    if puede_mover_arriba && !puede_mover_abajo{
        rango = (-max_pasos)..(0)
    }else if !puede_mover_arriba && puede_mover_abajo{
        rango = (0)..(max_pasos+1)
    }else{
        rango = (-max_pasos)..(max_pasos+1)
    }

    for i in rango{
        

        // Diagonal de izquierda a derecha y de arriba hacia abajo 
        if posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x() + i, posicion_atacante.get_y() + i){
            captura_pieza = true;
            break;
        }

        // Diaogonal de derecha a izquierda y de arriba hacia abajo
        else if posicion_receptor.coinciden_coordenadas(posicion_atacante.get_x() - i, posicion_atacante.get_y() + i){
            captura_pieza = true;
            break;
        }
    }
    captura_pieza
}
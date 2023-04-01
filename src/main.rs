use std::env;
use std::fs;

mod tablero;
mod pieza;
mod posicion;
mod movimiento;

use pieza::crear_pieza;
use tablero::{armar_tablero, buscar_pieza_blanca, buscar_pieza_negra};

/*
El output sera un caracter impreso por terminal:

    B: indica que solo la pieza blanca pueden capturar.
    N: indica que solo la pieza negra pueden capturar.
    E: indica que ambas piezas pueden capturar.
    P: indica que ninguna pieza puede capturar.
*/
fn formato_impresion(blanca_gana:bool, negra_gana:bool){  
        if blanca_gana && negra_gana{
            println!("E");
        }else if blanca_gana{
            println!("B");
        }else if negra_gana{
            println!("N");
        }else{
            println!("P");
        }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let filepath = &args[1];    

    // Leo el archivo pasado por parametro
    let contenido = 
        if let Ok(archivo) = fs::read_to_string(filepath){
         archivo   
        }else{
            print!("ERROR: Archivo invalido. Error en lectura");
            return;
        };

    let tablero: Vec<Vec<char>> = armar_tablero(&contenido);

    let pieza_blanca = 
        if let Some(pieza) = crear_pieza(&tablero,buscar_pieza_blanca(&tablero), "blanco".to_string()) {
            pieza
        } else {
            print!(" ERROR: No se pudo crear la pieza blanca. Revisar que se encuentre en el archivo");
            return;
        };        

    let pieza_negra = 
        if let Some(pieza) = crear_pieza(&tablero,buscar_pieza_negra(&tablero), "negro".to_string()) {
            pieza
        } else {
            print!("ERROR: No se pudo crear la pieza negra. Revisar que se encuentre en el archivo");
            return;
        }; 
    
    formato_impresion( pieza_blanca.captura(&pieza_negra), pieza_negra.captura(&pieza_blanca) )
}
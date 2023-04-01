//! # Ajedrez
//! #### Luis Jose Paredes Ramirez - lparedesr@fi.uba.ar - 104851
//!
//! ## Introducción
//! La presente entrega corresponde al primer trabajo practica
//! para Taller de Programación I - curso Deymonnaz.
//!
//! ## Objetivo
//! Determinar si dos piezas cualesquiera de ajedrez se pueden comer entre si dadas sus
//! posiciones en un tablero de ajedrez, implementandolo en Rust.
//!
//! Imprimir por terminal:
//!
//!    * B: indica que solo la pieza blanca pueden capturar.
//!    * N: indica que solo la pieza negra pueden capturar.
//!    * E: indica que ambas piezas pueden capturar.
//!    * P: indica que ninguna pieza puede capturar.
//!
//!
//! ## Ejecución
//! Para poder correr el programa se debe ejecutar el siguiente comando:
//!
//! cargo run -- <- archivo ->
//!
//! En particular se cuenta con 4 archivos de prueba: tabla_1.txt, tabla_2.txt, tabla_3.txt y tabla_4.txt
//!
//! Por ejemplo
//!
//! cargo run -- tabla_1.txt
//!
//! Comandos utiles:
//! - *cargo test*: Ejecuto los tests unitarios y de integración.
//! - *cargo fmt*: Formateo el código.
//! - *cargo clippy*:
//! - *cargo doc --open*: Abre la documentacion en el navegador.

use std::env;
use std::fs;

mod movimiento;
mod pieza;
mod posicion;
mod tablero;
mod result;

use result::{Ganador,Error};
use pieza::Pieza;
use tablero::Tablero;

/*
El output sera un caracter impreso por terminal:

    B: indica que solo la pieza blanca pueden capturar.
    N: indica que solo la pieza negra pueden capturar.
    E: indica que ambas piezas pueden capturar.
    P: indica que ninguna pieza puede capturar.
*/
fn formato_impresion(blanca_gana: bool, negra_gana: bool) -> char {
    let mut resultado:char = 'P';
    
    if blanca_gana && negra_gana {
        resultado = 'E';
    } else if blanca_gana {
        resultado = 'B';
    } else if negra_gana {
        resultado = 'N';
    }
    println!("{}", resultado);
    resultado
}

fn main() -> Result< Ganador,Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("ERROR: Se debe pasar un archivo como parametro");
        return Err(Error::FaltaParametro(String::from("ERROR: Se debe pasar un archivo como parametro")))
    }

    let filepath = "tablas/".to_owned() + &args[1]; // Las tablas estan en la carpeta tablas. Ejemplo tablas/tabla_1.txt

    let contenido = if let Ok(archivo) = fs::read_to_string(filepath) {
        archivo
    } else {
        print!("ERROR: Archivo invalido. Error en lectura");
        return Err(Error::ArchivoInvalido(String::from("ERROR: Archivo invalido. Error en lectura")))
    };

    let tablero = Tablero::new(&contenido);

    let pieza_blanca = if let Some(pieza) = Pieza::new(
        tablero.get_pieza_blanca(),
        tablero.posicion_pieza_blanca(),
        "blanco".to_string(),
    ) {
        pieza
    } else {
        print!(" ERROR: No se pudo crear la pieza blanca. Revisar que se encuentre en el archivo");
        return Err(Error::PiezaBlancaAusente(String::from("ERROR: No se pudo crear la pieza blanca. Revisar que se encuentre en el archivo")))
    };

    let pieza_negra = if let Some(pieza) = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    ) {
        pieza
    } else {
        print!("ERROR: No se pudo crear la pieza negra. Revisar que se encuentre en el archivo");
        return Err(Error::PiezaNegraAusente(String::from("ERROR: No se pudo crear la pieza negra. Revisar que se encuentre en el archivo")))
    };

    Ok(Ganador(formato_impresion(
        pieza_blanca.captura(&pieza_negra),
        pieza_negra.captura(&pieza_blanca) )
        ) 
    )
}

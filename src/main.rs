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

use pieza::Pieza;
use tablero::Tablero;

/*
El output sera un caracter impreso por terminal:

    B: indica que solo la pieza blanca pueden capturar.
    N: indica que solo la pieza negra pueden capturar.
    E: indica que ambas piezas pueden capturar.
    P: indica que ninguna pieza puede capturar.
*/
fn formato_impresion(blanca_gana: bool, negra_gana: bool) {
    if blanca_gana && negra_gana {
        println!("E");
    } else if blanca_gana {
        println!("B");
    } else if negra_gana {
        println!("N");
    } else {
        println!("P");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("ERROR: Se debe pasar un archivo como parametro");
        return;
    }

    let filepath = "tablas/".to_owned() + &args[1]; // Las tablas estan en la carpeta tablas. Ejemplo tablas/tabla_1.txt

    let contenido = if let Ok(archivo) = fs::read_to_string(filepath) {
        archivo
    } else {
        print!("ERROR: Archivo invalido. Error en lectura");
        return;
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
        return;
    };

    let pieza_negra = if let Some(pieza) = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    ) {
        pieza
    } else {
        print!("ERROR: No se pudo crear la pieza negra. Revisar que se encuentre en el archivo");
        return;
    };

    formato_impresion(
        pieza_blanca.captura(&pieza_negra),
        pieza_negra.captura(&pieza_blanca),
    )
}

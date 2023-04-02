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
//!
//! ## Modulos
//! 
//! * módulo [Movimiento](movimiento):
//! En este modulo se definen los movimientos de cada piezas
//! * módulo [Pieza](pieza):
//! En este modulo me encargo de crear las piezas y definir comportamiento (atacar a otra pieza)
//! * módulo [Posicion](posicion):
//! Este módulo contiene la estructura Posicion y sus métodos.
//! * módulo [Tablero](tablero):
//! Este modulo contiene la creacion del tablero de ajedrez, y la busqueda de las piezas en el mismo.
//! * módulo [Error](error):
//! Distintos tipos de errores que pueden suceder en el programa.

// Movimiento de las piezas
pub mod movimiento;

// Piezas de ajedrez
pub mod pieza;

// posicion en el tablero
pub mod posicion;

// tablero de ajedrez
pub mod tablero;

// Resultado de un partido
pub mod error;
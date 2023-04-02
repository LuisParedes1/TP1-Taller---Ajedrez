//! # Error
//!
//! Distintos tipos de errores que pueden suceder en el programas:
//!
//! * PiezaBlancaAusente: No se pudo crear la pieza blanca porque no se encontro en el tablero.
//! * PiezaNegraAusente: No se pudo crear la pieza negra porque no se encontro en el tablero.
//! * FaltaParametro: Se corrio el programa sin pasar el nombre del archivo que contiene el tablero.
//! * ArchivoInvalido: No se encontro el archivo en la carpeta src/tablas/

/// Distintos tipos de errores: PiezaBlancaAusente, PiezaNegraAusente, FaltaParametro & ArchivoInvalido
#[derive(Debug)]
pub enum Error {
    PiezaBlancaAusente(String),
    PiezaNegraAusente(String),
    FaltaParametro(String),
    ArchivoInvalido(String),
}

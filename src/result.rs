//! Resultados de la ejecución del programa.
//! 
//! El devuelve el ganador (N,B,E o P) y en caso de haber un error imprime el mensaje en terminal y devuelve el mensaje apropiado

#[derive(Debug)]
pub enum Error {
    PiezaBlancaAusente(String),
    PiezaNegraAusente(String),
    FaltaParametro(String),
    ArchivoInvalido(String),
}
#[derive(Debug)]
pub struct Ganador(pub char);

impl std::process::Termination for Ganador {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::SUCCESS
    }
}
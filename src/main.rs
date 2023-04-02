use std::env;
use std::fs;


use ej_individual::error::Error;
use ej_individual::pieza::Pieza;
use ej_individual::tablero::Tablero;

/*
El output sera un caracter impreso por terminal:

    B: indica que solo la pieza blanca pueden capturar.
    N: indica que solo la pieza negra pueden capturar.
    E: indica que ambas piezas pueden capturar.
    P: indica que ninguna pieza puede capturar.
*/
fn formato_impresion(blanca_gana: bool, negra_gana: bool) {
    let mut resultado:char = 'P';
    
    if blanca_gana && negra_gana {
        resultado = 'E';
    } else if blanca_gana {
        resultado = 'B';
    } else if negra_gana {
        resultado = 'N';
    }
    println!("{}", resultado);
}

fn main() -> Result< (),Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::FaltaParametro(String::from("Se debe pasar el nombre del archivo como parametro")))
    }

    let filepath = "tablas/".to_owned() + &args[1]; // Las tablas estan en la carpeta tablas. Ejemplo tablas/tabla_1.txt

    let contenido = if let Ok(archivo) = fs::read_to_string(filepath) {
        archivo
    } else {
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
        return Err(Error::PiezaBlancaAusente(String::from("ERROR: No se pudo crear la pieza blanca. Revisar que se encuentre en el archivo")))
    };

    let pieza_negra = if let Some(pieza) = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    ) {
        pieza
    } else {
        return Err(Error::PiezaNegraAusente(String::from("ERROR: No se pudo crear la pieza negra. Revisar que se encuentre en el archivo")))
    };

    formato_impresion(
        pieza_blanca.captura(&pieza_negra),
        pieza_negra.captura(&pieza_blanca) );
         
    Ok(())
}

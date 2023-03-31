use std::env;
use std::fs;

mod tablero;
use tablero::armar_tablero;

const UN_PASO: i8 = 1;
const SIN_LIM_PASOS: i8 = 7;

enum Pieza{
    Peon(Posicion, String),
    Caballo(Posicion, String),
    Alfil(Posicion, String),
    Torre(Posicion, String),
    Dama(Posicion, String),
    Rey(Posicion, String),
}

struct Posicion{
    x: i8,
    y: i8,
}

/*
    Devuelve si la pieza se come a un contrincante o no
*/
impl Pieza {
    fn captura(&self, contrincante:&Pieza) -> bool{
        match self {
            Pieza::Peon(posicion_atacante, color) => {
                    mover_peon(&posicion_atacante, contrincante.get_posicion(), color)
                },
            Pieza::Caballo(posicion_atacante, _) => {
                    mover_caballo(&posicion_atacante, contrincante.get_posicion())
                },
            Pieza::Alfil(posicion_atacante, _) => {
                    mover_alfil(&posicion_atacante, contrincante.get_posicion())
                },
            Pieza::Torre(posicion_atacante, _) => {
                    mover_torre(&posicion_atacante, contrincante.get_posicion())
                },
            Pieza::Dama(posicion_atacante,_) => {
                    mover_dama(&posicion_atacante, contrincante.get_posicion())
                },
            Pieza::Rey(posicion_atacante, _) => {
                    mover_rey(&posicion_atacante, contrincante.get_posicion())
                },
        }
    }

    fn get_posicion(&self) -> &Posicion{
        match self {
            Pieza::Peon(posicion,..) => {
                    posicion
                },
            Pieza::Caballo(posicion,..) => {
                    posicion
                },
            Pieza::Alfil(posicion,..) => {
                    posicion
                },
            Pieza::Torre(posicion,..) => {
                    posicion
                },
            Pieza::Dama(posicion,..) => {
                    posicion
                },
            Pieza::Rey(posicion,..) => {
                    posicion
                },
        }
    }

}

fn mover_rey(posicion_atacante:&Posicion, posicion_receptor: &Posicion)-> bool{
    mover_horizontal(posicion_atacante, posicion_receptor, UN_PASO) || 
        mover_vertical(posicion_atacante, posicion_receptor, UN_PASO) || 
            mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, true, true)
}


fn mover_dama(posicion_atacante:&Posicion, posicion_receptor: &Posicion)-> bool{
    mover_horizontal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS) || 
        mover_vertical(posicion_atacante, posicion_receptor, SIN_LIM_PASOS) || 
            mover_diagonal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS, true, true)
}

fn mover_torre(posicion_atacante:&Posicion, posicion_receptor: &Posicion)-> bool{
    mover_horizontal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS) || 
        mover_vertical(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
}

fn mover_alfil(posicion_atacante:&Posicion, posicion_receptor: &Posicion)-> bool{
    mover_diagonal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS, true, true)
}

fn mover_peon(posicion_atacante:&Posicion, posicion_receptor: &Posicion, color:&String)-> bool{
    if color == "blanco"{
        mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, true, false)
    }else {
        mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, false, true)
    }
}

fn mover_caballo(posicion_atacante:&Posicion, posicion_receptor: &Posicion) -> bool{

    let mut captura_pieza = false;
    
    
    for i in 0..2{
        
        // Uno/Dos derecha y Uno/Dos para arriba o para abajo
        if ( coinciden_x(posicion_atacante.x + 2-i , posicion_receptor.x) && 
                coinciden_y(posicion_atacante.y + 1+i , posicion_receptor.y)
            ) || 
            ( coinciden_x(posicion_atacante.x + 2-i , posicion_receptor.x) && 
                coinciden_y(posicion_atacante.y - (1+i), posicion_receptor.y) ){
                captura_pieza = true;
        }

        // Uno/Dos izquierda y Uno/Dos para arriba o para abajo
        else if ( coinciden_x(posicion_atacante.x - (2+i) , posicion_receptor.x) && 
                    coinciden_y(posicion_atacante.y + 1+i , posicion_receptor.y) 
                ) || 
            ( coinciden_x(posicion_atacante.x - (2+i) , posicion_receptor.x) && 
                coinciden_y(posicion_atacante.y - (1+i), posicion_receptor.y) ){
                captura_pieza = true;
        }
    }
    captura_pieza
}

/*
    Indica si la componente en X coincide para ambas posiciones
*/
fn coinciden_x(x_atacante:i8, x_receptor:i8)-> bool{
    x_atacante == x_receptor
}

/*
    Indica si la componente en Y coincide para ambas posiciones
*/
fn coinciden_y(y_atacante:i8, y_receptor:i8)-> bool{
    y_atacante == y_receptor
}


fn mover_diagonal(posicion_atacante:&Posicion, posicion_receptor: &Posicion, max_pasos:i8, puede_mover_arriba:bool, puede_mover_abajo:bool)-> bool{

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
        if coinciden_x(posicion_atacante.x + i, posicion_receptor.x) &&
                coinciden_y( posicion_atacante.y + i, posicion_receptor.y ){
                    captura_pieza = true;
                    break;
        }

        // Diaogonal de derecha a izquierda y de arriba hacia abajo
        else if coinciden_x(posicion_atacante.x - i, posicion_receptor.x) && 
                    coinciden_y(posicion_atacante.y + i , posicion_receptor.y){
                        captura_pieza = true;
                        break;
        }
    }
    captura_pieza

}

/*
    Me desplazo horizontalmente en el tablero y si las coordenadas coinciden entonces la pieza se come
    Devuelvo si la pieza atacante come a la pieza receptor
*/
fn mover_horizontal(posicion_atacante:&Posicion, posicion_receptor: &Posicion, cant_pasos:i8)-> bool{

        let mut come_pieza:bool = false;
    
        for i in 1..(cant_pasos+1){
            
            // derecha
            if coinciden_x(posicion_atacante.x, posicion_receptor.x) && 
                coinciden_y(posicion_atacante.y + i as i8, posicion_receptor.y){
                    come_pieza = true;
                    break;
            }
            
            // izquierda
            else if coinciden_x(posicion_atacante.x, posicion_receptor.x) && 
                coinciden_y(posicion_atacante.y - i as i8, posicion_receptor.y){
                    come_pieza = true;
                    break;
            }
        }    
        come_pieza
}

/*
    Me desplazo verticalmente en el tablero y si las coordenadas coinciden entonces la pieza se come.
    Devuelvo si la pieza atacante come a la pieza receptor
*/
fn mover_vertical(posicion_atacante:&Posicion, posicion_receptor: &Posicion, cant_pasos:i8)-> bool{

    let mut come_pieza:bool = false;

    for i in 1..(cant_pasos+1){
        
        // arriba
        if coinciden_x(posicion_atacante.x - i as i8, posicion_receptor.x) && 
            coinciden_y(posicion_atacante.y, posicion_receptor.y){
                come_pieza = true;
                break;
        }
        
        // abajo
        else if coinciden_x(posicion_atacante.x + i as i8, posicion_receptor.x) && 
            coinciden_y(posicion_atacante.y, posicion_receptor.y){
                come_pieza = true;
                break;
        }
    }    
    come_pieza
}

/*
    Busco en el tablero la posicion de la pieza blanca (en mayuscula)

    En caso de no encontrar devuelve la posicion (0,0) y luego cuando creo la pieza
    devuelve error en caso de no encontrarse la pieza en esa posicion defecto
*/
fn obtener_posicion_blanca(tablero: &Vec<Vec<char>>) -> Posicion{

    let mut posicion: Posicion = Posicion{x: 0, y: 0};

    for (i, fila) in tablero.iter().enumerate() {
        for (j, c) in fila.iter().enumerate() {
            if c.is_ascii_uppercase(){
                posicion.x = i as i8;
                posicion.y = j as i8;
            }
        }
    }

    posicion
}

/*
    Busco en el tablero la posicion de la pieza negra (en minuscula)

    En caso de no encontrar devuelve la posicion (0,0) y luego cuando creo la pieza
    devuelve error en caso de no encontrarse la pieza en esa posicion defecto
*/
fn obtener_posicion_negra(tablero: &Vec<Vec<char>>) -> Posicion{

    let mut posicion: Posicion = Posicion{x: 0, y: 0};

    for (i, fila) in tablero.iter().enumerate() {
        for (j, c) in fila.iter().enumerate() {
            if c.is_ascii_lowercase(){
                posicion.x = i as i8;
                posicion.y = j as i8;
            }
        }
    }

    posicion
}

/*
    Devuelve la pieza que se encuentra en la posicion indicada. 
    
    En caso de no coincidir con alguna pieza (Dama, Rey, Peon, Alfi, Caballo, Torre) devuelve un None.
*/
fn crear_pieza(tablero: &Vec<Vec<char>>, posicion: Posicion, color:String) -> Option<Pieza> {

    let pieza:char = tablero[posicion.x as usize][posicion.y as usize];

    match pieza {
        'R' | 'r' => Some( Pieza::Rey(Posicion::from(posicion), color) ),
        'D' | 'd' => Some( Pieza::Dama(Posicion::from(posicion), color) ),
        'A' | 'a' => Some( Pieza::Alfil(Posicion::from(posicion), color) ),
        'C' | 'c' => Some( Pieza::Caballo(Posicion::from(posicion), color) ),
        'T' | 't' => Some( Pieza::Torre(Posicion::from(posicion), color) ),
        'P' | 'p' => Some( Pieza::Peon(Posicion::from(posicion), color) ),
        _ => None,
    }
}

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
            print!("Archivo invalido. Error en lectura");
            return;
        };

    // Creo mi matriz del juego a partir del contenido leido
    let tablero: Vec<Vec<char>> = armar_tablero(&contenido);

    let pieza_blanca = 
        if let Some(pieza) = crear_pieza(&tablero,obtener_posicion_blanca(&tablero), "blanco".to_string()) {
            pieza
        } else {
            print!("No se pudo crear la pieza blanca. Revisar que se encuentre en el archivo");
            return;
        };        

    let pieza_negra = 
        if let Some(pieza) = crear_pieza(&tablero,obtener_posicion_negra(&tablero), "negro".to_string()) {
            pieza
        } else {
            print!("No se pudo crear la pieza negra. Revisar que se encuentre en el archivo");
            return;
        }; 
        
    
    formato_impresion( pieza_blanca.captura(&pieza_negra), pieza_negra.captura(&pieza_blanca) )
    //formato_impresion(pieza_blanca.mover(&tablero),pieza_negra.mover(&tablero));

}
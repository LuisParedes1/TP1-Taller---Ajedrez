use std::env;
use std::fs;

mod tablero;
use tablero::armar_tablero;

enum Pieza{
    Peon(DatosPieza),
    Caballo(DatosPieza),
    Alfil(DatosPieza),
    Torre(DatosPieza),
    Dama(DatosPieza),
    Rey(DatosPieza),
}

/*
    Devuelve si la pieza se come a un contrincante o no
*/
impl Pieza {
    fn mover(&self, tablero: &Vec<Vec<char>>) -> bool{
        match self {
            Pieza::Peon(datos) => {
                    mover_peon(tablero, &datos)
                },
            Pieza::Caballo(datos) => {
                    mover_caballo(tablero, &datos)
                },
            Pieza::Alfil(datos) => {
                    mover_alfil(tablero, &datos)
                },
            Pieza::Torre(datos) => {
                    mover_torre(tablero, &datos)
                },
            Pieza::Dama(datos) => {
                    mover_dama(tablero, &datos)
                },
            Pieza::Rey(datos) => {
                    mover_rey(tablero, &datos)
                },
        }
    }
}

fn mover_caballo(tablero: &Vec<Vec<char>>, datos:&DatosPieza) -> bool{
    let x:usize = datos.posicion.x;
    let y:usize = datos.posicion.y;
    
    let mut come_pieza = false;

    if x+2 < 8{
        if y+1 < 8{
            if tablero[x+2][y+1].is_alphabetic(){
                come_pieza = true;
            }
        }
        if y as i8 - 1 >= 0{
            if tablero[x+2][y-1].is_alphabetic(){
                come_pieza = true;
            }
        }
    }

    if x+1 < 8{
        if y+2 < 8{
            if tablero[x+1][y+2].is_alphabetic(){
                come_pieza = true;
            }
        }
        if y as i8 - 2 >= 0{
            if tablero[x+1][y-2].is_alphabetic(){
                come_pieza = true;
            }
        }
    }

    if x as i8 - 2 >= 0{
        if y+1 < 8{
            if tablero[x-2][y+1].is_alphabetic(){
                come_pieza = true;
            }
        }
        if y as i8 - 1 >= 0{
            if tablero[x-2][y-1].is_alphabetic(){
                come_pieza = true;
            }
        }
    }

    if x as i8 - 1 >= 0{
        if y+2 < 8{
            if tablero[x-1][y+2].is_alphabetic(){
                come_pieza = true;
            }
        }
        if y as i8 - 2 >= 0{
            if tablero[x-1][y-2].is_alphabetic(){
                come_pieza = true;
            }
        }
    }

    come_pieza
}

fn mover_rey(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{
    mover_horizontal(tablero, &datos) || mover_vertical(tablero, &datos) || mover_diagonal(tablero, &datos)
}


fn mover_dama(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{
    mover_horizontal(tablero, &datos) || mover_vertical(tablero, &datos) || mover_diagonal(tablero, &datos)
}

fn mover_torre(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{
    mover_horizontal(tablero, &datos) || mover_vertical(tablero, &datos)
}

fn mover_alfil(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{
    mover_diagonal(tablero, &datos)
}

fn mover_peon(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{
    let x:usize = datos.posicion.x;
    let y:usize = datos.posicion.y;

    let mut come_pieza:bool = false;

    if x > 0 && x < 7 && y > 0 && y < 7{    // Si la pieza esta entre la fila 1-6 y la columna 1-6

        if datos.color == "blanca"{ // Si es blanca va hacia arriba
            if tablero[x+1][y+1].is_alphabetic() || tablero[x-1][y+1].is_alphabetic() {
                come_pieza = true;
            }
        }else{ // Si es Negra va hacia abajo
            if tablero[x+1][y-1].is_alphabetic() || tablero[x-1][y-1].is_alphabetic() {
                come_pieza = true;
            }
        }
    }else if x == 0 && y > 0 && y < 7{  // Si esta en la columna 0

        if datos.color == "blanca"{ // Si es blanca va hacia arriba
            if tablero[x+1][y+1].is_alphabetic() {
                come_pieza = true;
            }
        }else{ // Si es Negra va hacia abajo
            if tablero[x+1][y-1].is_alphabetic() {
                come_pieza = true;
            }
        }
    }else if x == 7 && y > 0 && y < 7{  // Si esta en la columna 7

        if datos.color == "blanca"{ // Si es blanca va hacia arriba
            if tablero[x-1][y+1].is_alphabetic() {
                come_pieza = true;
            }
        }else{ // Si es Negra va hacia abajo
            if tablero[x-1][y-1].is_alphabetic() {
                come_pieza = true;
            }
        }
    }

    come_pieza
}


fn mover_diagonal(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{

    let x:usize = datos.posicion.x;
    let y:usize = datos.posicion.y;

    let mut come_pieza:bool = false;

    let mut max_pasos = 8;

    if datos.mueve_a_un_paso{
        max_pasos = 2;
    }


    for i in 1..max_pasos{

        // derecha y abajo
        if x+i < 8 && y+i < 8{
            if tablero[x+i][y+i].is_alphabetic(){
                come_pieza = true;
            }
        }

        // derecha y arriba
        if x+i < 8 && y as i8 - i as i8 >= 0{
            if tablero[x+i][y-i].is_alphabetic(){
                come_pieza = true;
            }
        }

        // izquierda y arriba
        if x as i8 - i as i8 >=0 && y as i8 - i as i8 >= 0{
            if tablero[x-i][y-i].is_alphabetic(){
                come_pieza = true;
            }
        }

        // izquierda y abajo
        if x as i8 - i as i8 >=0 && y+i < 8{
            if tablero[x-i][y+i].is_alphabetic(){
                come_pieza = true;
            }
        }
    }    
    come_pieza

}

fn mover_horizontal(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{
    
        let x:usize = datos.posicion.x;
        let y:usize = datos.posicion.y;
    
        let mut come_pieza:bool = false;

        let mut max_pasos = 8;
    
        if datos.mueve_a_un_paso{
            max_pasos = 2;
        }
    
    
        for i in 1..max_pasos{
    
            // derecha
            if y+i < 8{
                if tablero[x][y+i].is_alphabetic(){
                    come_pieza = true;
                }
            }
    
            // izquierda
            if y as i8 - i as i8 >= 0{
                if tablero[x][y-i].is_alphabetic(){
                    come_pieza = true;
                }
            }
        }    
        come_pieza
    
}

fn mover_vertical(tablero: &Vec<Vec<char>>, datos:&DatosPieza)-> bool{
    
        let x = datos.posicion.x;
        let y= datos.posicion.y;
    
        let mut come_pieza:bool = false;
    
        let mut max_pasos = 8;
    
        if datos.mueve_a_un_paso{
            max_pasos = 2;
        }
    
    
        for i in 1..max_pasos{
    
            // Bajo 
            if x+i < 8{
                if tablero[x+i][y].is_alphabetic(){
                    come_pieza = true;
                }
            }
    
            // Subo
            if (x as i8 - i as i8) >= 0{
                if tablero[x-i][y].is_alphabetic(){
                    come_pieza = true;
                }
            }
        }    
        come_pieza
    
}


struct DatosPieza{
    color: String,
    posicion: Posicion,
    mueve_a_un_paso:bool,
}

struct Posicion{
    x: usize,
    y: usize,
}


fn obtener_posicion_blanca(tablero: &Vec<Vec<char>>) -> Posicion{

    let mut posicion: Posicion = Posicion{x: 0, y: 0};

    for (i, fila) in tablero.iter().enumerate() {
        for (j, c) in fila.iter().enumerate() {
            if c.is_ascii_uppercase(){
                posicion.x = i;
                posicion.y = j;
            }
        }
    }

    posicion
}

fn obtener_posicion_negra(tablero: &Vec<Vec<char>>) -> Posicion{

    let mut posicion: Posicion = Posicion{x: 0, y: 0};

    for (i, fila) in tablero.iter().enumerate() {
        for (j, c) in fila.iter().enumerate() {
            if c.is_ascii_lowercase(){
                posicion.x = i;
                posicion.y = j;
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

    let pieza:char = tablero[posicion.x][posicion.y];

    match pieza {
        'R' | 'r' => Some( Pieza::Rey(DatosPieza{color: color, posicion: posicion, mueve_a_un_paso:true}) ),
        'D' | 'd' => Some( Pieza::Dama(DatosPieza{color: color, posicion: posicion, mueve_a_un_paso:false}) ),
        'A' | 'a' => Some( Pieza::Alfil(DatosPieza{color: color, posicion: posicion, mueve_a_un_paso:false}) ),
        'C' | 'c' => Some( Pieza::Caballo(DatosPieza{color: color, posicion: posicion, mueve_a_un_paso:false}) ),
        'T' | 't' => Some( Pieza::Torre(DatosPieza{color: color, posicion: posicion, mueve_a_un_paso:false}) ),
        'P' | 'p' => Some( Pieza::Peon(DatosPieza{color: color, posicion: posicion, mueve_a_un_paso:false}) ),
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
        
    
    formato_impresion(pieza_blanca.mover(&tablero),pieza_negra.mover(&tablero));

}
#[derive(Debug)]
enum Piezas{
    Peon(DatosPieza),
    Caballo(DatosPieza),
    Alfil(DatosPieza),
    Torre(DatosPieza),
    Reina(DatosPieza),
    Rey(DatosPieza),
    None,
}

#[derive(Debug)]
struct DatosPieza{
    color: String,
    posicion: Posicion,
    come_pieza: bool,
}

#[derive(Debug)]
struct Posicion{
    x: usize,
    y: usize,
}




pub fn crearPieza(tablero: &Vec<Vec<char>>, posicion: Posicion, color:String) -> Pieza {

    let pieza:char = tablero[posicion.x][posicion.y];

    match pieza {
        'R' | 'r' => Pieza::Rey(DatosPieza{color: color, posicion: posicion, come_pieza: false}),
        'D' | 'd' => Pieza::Dama(DatosPieza{color: color, posicion: posicion, come_pieza: false}),
        'A' | 'a' => Pieza::Alfil(DatosPieza{color: color, posicion: posicion, come_pieza: false}),
        'C' | 'c' => Pieza::Caballo(DatosPieza{color: color, posicion: posicion, come_pieza: false}),
        'T' | 't' => Pieza::Torre(DatosPieza{color: color, posicion: posicion, come_pieza: false}),
        'P' | 'p' => Pieza::Peon(DatosPieza{color: color, posicion: posicion, come_pieza: false}),
        _ => Pieza::None,
    }
}
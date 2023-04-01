use crate::posicion::Posicion;

/*
    Armo el tablero de ajedrez
*/
pub fn armar_tablero(contents: &str) -> Vec<Vec<char>> {

    let mut tablero: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        
        let mut fila: Vec<char> = Vec::new();

        for c in line.chars() {
            if c != ' '{
                fila.push(c);
            }
        }
        
        tablero.push(fila);
    }

    tablero
}

/*
    Busco en el tablero la posicion de la pieza negra (en minuscula)

    En caso de no encontrar devuelve la posicion (0,0) y luego cuando creo la pieza
    devuelve error en caso de no encontrarse la pieza en esa posicion defecto
*/
pub fn buscar_pieza_negra(tablero: &Vec<Vec<char>>) -> Posicion{

    let mut posicion: Posicion = Posicion::new(0, 0);

    for (i, fila) in tablero.iter().enumerate() {
        for (j, c) in fila.iter().enumerate() {
            if c.is_ascii_lowercase(){
                posicion.set_posicion(i as i8,j as i8);
            }
        }
    }

    posicion
}

/*
    Busco en el tablero la posicion de la pieza blanca (en mayuscula)

    En caso de no encontrar devuelve la posicion (0,0) y luego cuando creo la pieza
    devuelve error en caso de no encontrarse la pieza en esa posicion defecto
*/
pub fn buscar_pieza_blanca(tablero: &Vec<Vec<char>>) -> Posicion{

    let mut posicion: Posicion = Posicion::new(0, 0);

    for (i, fila) in tablero.iter().enumerate() {
        for (j, c) in fila.iter().enumerate() {
            if c.is_ascii_uppercase(){
                posicion.set_posicion(i as i8,j as i8);
            }
        }
    }

    posicion
}
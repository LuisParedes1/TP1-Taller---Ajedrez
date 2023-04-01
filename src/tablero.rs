//! # Tablero
//!
//! El siguiente modulo contiene la creacion del tablero de ajedrez, y la busqueda de las piezas en el mismo.
//!
//! Siendo las piezas negras las que se encuentran en mayuscula, se devuelve la posicion de la primera pieza mayuscula encontrada.
//! Para las piezas blancas es el mismo caso pero con minuscula.
//!
//! En caso de no encontrarse ninguna pieza en el tablero se devolvera la posicion por defecto (0,0).
//!
//! Luego, al crearse la pieza se devolvera un error en caso de que no haya ninguna pieza en la misma posicion

use crate::posicion::Posicion;

pub struct Tablero {
    tablero: Vec<Vec<char>>,
}

impl Tablero {
    // Armo el tablero de ajedrez
    pub fn new(contents: &str) -> Tablero {
        let mut tablero: Vec<Vec<char>> = Vec::new();

        for line in contents.lines() {
            let mut fila: Vec<char> = Vec::new();

            for c in line.chars() {
                if c != ' ' {
                    fila.push(c);
                }
            }

            tablero.push(fila);
        }
        Tablero { tablero }
    }

    /*
        Busco en el tablero la pieza negra.

        En caso de no encontrarlo devuelve _ (que seria la posicion (0,0) del tablero)
    */
    pub fn buscar_pieza_negra(&self) -> Posicion {
        let mut posicion: Posicion = Posicion::new(0, 0);

        for (i, fila) in self.tablero.iter().enumerate() {
            for (j, c) in fila.iter().enumerate() {
                if c.is_ascii_uppercase() {
                    posicion.set_posicion(i as i8, j as i8);
                }
            }
        }
        posicion
    }

    // Devuelve la pieza negra encontrada
    pub fn get_pieza_negra(&self) -> char {
        self.get_pieza(self.buscar_pieza_negra())
    }

    /*
        Busco en el tablero la pieza blanca.

        En caso de no encontrarlo devuelve _ (que seria la posicion (0,0) del tablero)
    */
    pub fn buscar_pieza_blanca(&self) -> Posicion {
        let mut posicion: Posicion = Posicion::new(0, 0);

        for (i, fila) in self.tablero.iter().enumerate() {
            for (j, c) in fila.iter().enumerate() {
                if c.is_ascii_lowercase() {
                    posicion.set_posicion(i as i8, j as i8);
                }
            }
        }
        posicion
    }

    // Devuelve la pieza blanca encontrada
    pub fn get_pieza_blanca(&self) -> char {
        self.get_pieza(self.buscar_pieza_blanca())
    }

    // Devuelve la pieza que se encuentra en la posicion indicada
    pub fn get_pieza(&self, posicion: Posicion) -> char {
        self.tablero[posicion.get_x() as usize][posicion.get_y() as usize]
    }
}

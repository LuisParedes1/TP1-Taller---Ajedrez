use crate::posicion::Posicion;

use crate::movimiento::{mover_rey, mover_dama, mover_alfil, mover_caballo, mover_torre, mover_peon};


pub enum Pieza{
    Peon(Posicion, String),
    Caballo(Posicion, String),
    Alfil(Posicion, String),
    Torre(Posicion, String),
    Dama(Posicion, String),
    Rey(Posicion, String),
}

/*
    Devuelve la pieza que se encuentra en la posicion indicada. 
    
    En caso de no coincidir con alguna pieza (Dama, Rey, Peon, Alfi, Caballo, Torre) devuelve un None.
*/
pub fn crear_pieza(tablero: &Vec<Vec<char>>, posicion: Posicion, color:String) -> Option<Pieza> {

    let pieza:char = tablero[posicion.get_x() as usize][posicion.get_y() as usize];

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

impl Pieza {

    /*
        Devuelve si la pieza se come a un contrincante o no
    */
    pub fn captura(&self, contrincante:&Pieza) -> bool{
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

    /*
        Devuelve la posicion de la pieza actual
    */
    pub fn get_posicion(&self) -> &Posicion{
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
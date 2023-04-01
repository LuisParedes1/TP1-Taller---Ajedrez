mod caballo;
mod diagonal;
mod horizontal;
mod vertical;

use crate::posicion::Posicion;

// Movimiento de las piezas
use caballo::mover_l;
use diagonal::mover_diagonal;
use horizontal::mover_horizontal;
use vertical::mover_vertical;

const UN_PASO: i8 = 1;
const SIN_LIM_PASOS: i8 = 7;

pub fn mover_rey(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_horizontal(posicion_atacante, posicion_receptor, UN_PASO)
        || mover_vertical(posicion_atacante, posicion_receptor, UN_PASO)
        || mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, true, true)
}

pub fn mover_dama(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_horizontal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
        || mover_vertical(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
        || mover_diagonal(
            posicion_atacante,
            posicion_receptor,
            SIN_LIM_PASOS,
            true,
            true,
        )
}

pub fn mover_torre(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_horizontal(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
        || mover_vertical(posicion_atacante, posicion_receptor, SIN_LIM_PASOS)
}

pub fn mover_alfil(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_diagonal(
        posicion_atacante,
        posicion_receptor,
        SIN_LIM_PASOS,
        true,
        true,
    )
}

pub fn mover_peon(
    posicion_atacante: &Posicion,
    posicion_receptor: &Posicion,
    color: &String,
) -> bool {
    if color == "blanco" {
        mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, true, false)
    } else {
        mover_diagonal(posicion_atacante, posicion_receptor, UN_PASO, false, true)
    }
}

pub fn mover_caballo(posicion_atacante: &Posicion, posicion_receptor: &Posicion) -> bool {
    mover_l(posicion_atacante, posicion_receptor)
}

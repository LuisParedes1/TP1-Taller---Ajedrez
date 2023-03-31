use crate::posicion::Posicion;

const UN_PASO: i8 = 1;
const SIN_LIM_PASOS: i8 = 7;

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


/*
    Devuelve si la pieza se come a un contrincante o no
*/
impl Pieza {
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


fn mover_caballo(posicion_atacante:&Posicion, posicion_receptor: &Posicion) -> bool{

    let mut captura_pieza = false;
    
    
    for i in 0..2{
        
        // Uno/Dos derecha y Uno/Dos para arriba o para abajo
        if ( coinciden_x(posicion_atacante.get_x() + 2-i , posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() + 1+i , posicion_receptor.get_y())
            ) || 
            ( coinciden_x(posicion_atacante.get_x() + 2-i , posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() - (1+i), posicion_receptor.get_y()) ){
                captura_pieza = true;
        }

        // Uno/Dos izquierda y Uno/Dos para arriba o para abajo
        else if ( coinciden_x(posicion_atacante.get_x() - (2+i) , posicion_receptor.get_x()) && 
                    coinciden_y(posicion_atacante.get_y() + 1+i , posicion_receptor.get_y()) 
                ) || 
            ( coinciden_x(posicion_atacante.get_x() - (2+i) , posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() - (1+i), posicion_receptor.get_y()) ){
                captura_pieza = true;
        }
    }
    captura_pieza
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
        if coinciden_x(posicion_atacante.get_x() + i, posicion_receptor.get_x()) &&
                coinciden_y( posicion_atacante.get_y() + i, posicion_receptor.get_y() ){
                    captura_pieza = true;
                    break;
        }

        // Diaogonal de derecha a izquierda y de arriba hacia abajo
        else if coinciden_x(posicion_atacante.get_x() - i, posicion_receptor.get_x()) && 
                    coinciden_y(posicion_atacante.get_y() + i , posicion_receptor.get_y()){
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
            if coinciden_x(posicion_atacante.get_x(), posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() + i as i8, posicion_receptor.get_y()){
                    come_pieza = true;
                    break;
            }
            
            // izquierda
            else if coinciden_x(posicion_atacante.get_x(), posicion_receptor.get_x()) && 
                coinciden_y(posicion_atacante.get_y() - i as i8, posicion_receptor.get_y()){
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
        if coinciden_x(posicion_atacante.get_x() - i as i8, posicion_receptor.get_x()) && 
            coinciden_y(posicion_atacante.get_y(), posicion_receptor.get_y()){
                come_pieza = true;
                break;
        }
        
        // abajo
        else if coinciden_x(posicion_atacante.get_x() + i as i8, posicion_receptor.get_x()) && 
            coinciden_y(posicion_atacante.get_y(), posicion_receptor.get_y()){
                come_pieza = true;
                break;
        }
    }    
    come_pieza
}
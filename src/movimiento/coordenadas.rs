/*
    Indica si la componente en X coincide para ambas posiciones
*/
pub fn coinciden_x(x_atacante:i8, x_receptor:i8)-> bool{
    x_atacante == x_receptor
}

/*
    Indica si la componente en Y coincide para ambas posiciones
*/
pub fn coinciden_y(y_atacante:i8, y_receptor:i8)-> bool{
    y_atacante == y_receptor
}

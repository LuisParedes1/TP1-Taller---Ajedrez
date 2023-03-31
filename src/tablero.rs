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
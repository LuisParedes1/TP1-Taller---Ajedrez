use ej_individual::pieza::Pieza;
use ej_individual::tablero::Tablero;

fn formato_impresion(blanca_gana: bool, negra_gana: bool) -> String {
    let mut resultado: char = 'P';

    if blanca_gana && negra_gana {
        resultado = 'E';
    } else if blanca_gana {
        resultado = 'B';
    } else if negra_gana {
        resultado = 'N';
    }
    resultado.to_string()
}

#[test]
fn test_dado_talero_01_espero_devuelva_n() {
    let resultado_esperado = "N";
    let contenido = std::fs::read_to_string("tablas/table_1.txt").unwrap();

    let tablero = Tablero::new(&contenido);

    let pieza_blanca = Pieza::new(
        tablero.get_pieza_blanca(),
        tablero.posicion_pieza_blanca(),
        "blanco".to_string(),
    )
    .unwrap();

    let pieza_negra = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    )
    .unwrap();

    assert_eq!(
        resultado_esperado,
        formato_impresion(
            pieza_blanca.captura(&pieza_negra),
            pieza_negra.captura(&pieza_blanca)
        )
    );
}

#[test]
fn test_dado_talero_02_espero_devuelva_b() {
    let resultado_esperado = "B";
    let contenido = std::fs::read_to_string("tablas/table_2.txt").unwrap();

    let tablero = Tablero::new(&contenido);

    let pieza_blanca = Pieza::new(
        tablero.get_pieza_blanca(),
        tablero.posicion_pieza_blanca(),
        "blanco".to_string(),
    )
    .unwrap();

    let pieza_negra = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    )
    .unwrap();

    assert_eq!(
        resultado_esperado,
        formato_impresion(
            pieza_blanca.captura(&pieza_negra),
            pieza_negra.captura(&pieza_blanca)
        )
    );
}

#[test]
fn test_dado_talero_03_espero_devuelva_n() {
    let resultado_esperado = "E";
    let contenido = std::fs::read_to_string("tablas/table_3.txt").unwrap();
    let tablero = Tablero::new(&contenido);

    let pieza_blanca = Pieza::new(
        tablero.get_pieza_blanca(),
        tablero.posicion_pieza_blanca(),
        "blanco".to_string(),
    )
    .unwrap();

    let pieza_negra = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    )
    .unwrap();

    assert_eq!(
        resultado_esperado,
        formato_impresion(
            pieza_blanca.captura(&pieza_negra),
            pieza_negra.captura(&pieza_blanca)
        )
    );
}

#[test]
fn test_dado_talero_04_espero_devuelva_n() {
    let resultado_esperado = "P";
    let contenido = std::fs::read_to_string("tablas/table_4.txt").unwrap();
    let tablero = Tablero::new(&contenido);

    let pieza_blanca = Pieza::new(
        tablero.get_pieza_blanca(),
        tablero.posicion_pieza_blanca(),
        "blanco".to_string(),
    )
    .unwrap();

    let pieza_negra = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    )
    .unwrap();

    assert_eq!(
        resultado_esperado,
        formato_impresion(
            pieza_blanca.captura(&pieza_negra),
            pieza_negra.captura(&pieza_blanca)
        )
    );
}

#[test]
fn test_dado_talero_sin_blancas_espero_none_al_crear_pieza_blanca() {
    let contenido = std::fs::read_to_string("tablas/tablero_sin_blancas.txt").unwrap();
    let tablero = Tablero::new(&contenido);

    let pieza_blanca = Pieza::new(
        tablero.get_pieza_blanca(),
        tablero.posicion_pieza_blanca(),
        "blanco".to_string(),
    );

    assert!(pieza_blanca.is_none());
}

#[test]
fn test_dado_talero_sin_negras_espero_none_al_crear_pieza_negra() {
    let contenido = std::fs::read_to_string("tablas/tablero_sin_negras.txt").unwrap();
    let tablero = Tablero::new(&contenido);

    let pieza_negra = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    );

    assert!(pieza_negra.is_none());
}

#[test]
fn test_dado_talero_sin_piezas_espero_none_al_crear_pieza_blanca() {
    let contenido = std::fs::read_to_string("tablas/tablero_sin_piezas.txt").unwrap();
    let tablero = Tablero::new(&contenido);

    let pieza_blanca = Pieza::new(
        tablero.get_pieza_blanca(),
        tablero.posicion_pieza_blanca(),
        "blanco".to_string(),
    );

    assert!(pieza_blanca.is_none());
}

#[test]
fn test_dado_talero_sin_piezas_espero_none_al_crear_pieza_negra() {
    let contenido = std::fs::read_to_string("tablas/tablero_sin_piezas.txt").unwrap();
    let tablero = Tablero::new(&contenido);

    let pieza_negra = Pieza::new(
        tablero.get_pieza_negra(),
        tablero.posicion_pieza_negra(),
        "negro".to_string(),
    );

    assert!(pieza_negra.is_none());
}

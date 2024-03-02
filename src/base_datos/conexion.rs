use std::path::Path;

use rusqlite::Connection;

/// Establecer conexión sqlite en bbdd.sqlite en el directorio del ejecutable
pub fn conectar(ruta_base: &Path) -> Connection {
    let ruta_base_datos = ruta_base.join("bbdd.sqlite");
    let conexion = rusqlite::Connection::open(ruta_base_datos)
        .expect("error crítico: no se ha podido establecer la conexión con la base de datos");

    conexion
}

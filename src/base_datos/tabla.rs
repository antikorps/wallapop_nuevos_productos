use rusqlite::Connection;

/// Crear la tabla wallapop (si no existe) en la base de datos
pub fn crear(conexion: &Connection) {
    let declaracion = r###"
    CREATE TABLE IF NOT EXISTS wallapop (
        "id" INTEGER NOT NULL,
	    "wallapop_id" TEXT NOT NULL,
	    PRIMARY KEY("id" AUTOINCREMENT)
      );
    "###;
    conexion
        .execute(declaracion, ())
        .expect("error crítico: ha fallado la consulta create table");
}

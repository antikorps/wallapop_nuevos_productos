use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuracion {
    pub cliente_user_agent: String,
    pub telegram_bot_token: String,
    pub telegram_chat: String,
    pub telegram_espera: u64,
    pub telegram_notificar: bool,
    pub wallapop_urls: Vec<String>,
    pub wallapop_simultaneidad: u64,
    pub wallapop_espera: u64,
}

/// Procesar el archivo configuracion.json del directorio del ejecutable
pub fn procesar(ruta_base: &Path) -> Configuracion {
    let ruta_configuracion_json = ruta_base.join("configuracion.json");
    let configuracion_json = fs::File::open(ruta_configuracion_json).expect("error crítico: no se ha encontrado el archivo configuracion.json en el directorio del ejecutable");

    let configuracion: Configuracion = serde_json::from_reader(configuracion_json).expect("error crítico: ha fallado la deserialización de configuracion.json, comprueba que el archivo es válido");

    configuracion
}

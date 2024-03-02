use reqwest::Client;
use rusqlite::Connection;

use crate::configurar::archivo_configuracion::Configuracion;

pub struct Manejador {
    pub base_datos: Connection,
    pub cliente: Client,
    pub token: String,
    pub chat: String,
    pub telegram_espera: u64,
    pub wallapop_espera: u64,
    pub simultaneidad: u64,
    pub urls: Vec<String>,
    pub registros: Option<Vec<Registro>>,
    pub notificaciones: Option<Vec<Registro>>,
    pub notificar: bool,
}

pub struct Registro {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub precio: f64,
    pub moneda: String,
    pub webslug: String,
}

impl Manejador {
    pub fn new(configuracion: Configuracion, base_datos: Connection, cliente: Client) -> Manejador {
        Manejador {
            cliente,
            base_datos,
            token: configuracion.telegram_bot_token,
            chat: configuracion.telegram_chat,
            telegram_espera: configuracion.telegram_espera,
            wallapop_espera: configuracion.wallapop_espera,
            simultaneidad: configuracion.wallapop_simultaneidad,
            urls: configuracion.wallapop_urls,
            registros: None,
            notificaciones: None,
            notificar: configuracion.telegram_notificar,
        }
    }
}

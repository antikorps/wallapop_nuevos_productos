use std::time::Duration;

use reqwest::Client;

use super::crear::{Manejador, Registro};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespuestaApi {
    #[serde(rename = "search_objects")]
    pub search_objects: Vec<SearchObject>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchObject {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    #[serde(rename = "web_slug")]
    pub web_slug: String,
}

/// Consulta a la API de wallapop para recuperar los registros
pub async fn recuperar_registros_api(
    cliente: &Client,
    url: &String,
) -> Result<Vec<Registro>, String> {
    let r;
    match cliente.get(url).send().await {
        Err(error) => {
            let mensaje_error = format!("ATENCIÓN: error consultando {url}: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => r = ok,
    }
    if r.status() != 200 {
        let mensaje_error = format!(
            "ATENCIÓN: error consultando {url} debido a un status code incorrecto {}",
            r.status()
        );
        return Err(mensaje_error);
    };

    let respuesta;
    match r.text().await {
        Err(error) => {
            let mensaje_error =
                format!("ATENCIÓN: error obteniendo la respuesta de la consulta de {url}: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => respuesta = ok,
    }

    let deserializacion: Result<RespuestaApi, serde_json::Error> = serde_json::from_str(&respuesta);
    let respuesta_api;
    match deserializacion {
        Err(error) => {
            let mensaje_error = format!(
                "ATENCIÓN: error deserializando la respuesta de la consulta de {url}: {error}"
            );
            return Err(mensaje_error);
        }
        Ok(ok) => respuesta_api = ok,
    }

    if respuesta_api.search_objects.len() == 0 {
        let mensaje_error =
            format!("ATENCIÓN: la respuesta de la {url} no ha devuelto ningún registro");
        return Err(mensaje_error);
    }

    let mut registros = Vec::new();

    for v in respuesta_api.search_objects {
        let registro = Registro {
            id: v.id,
            titulo: v.title,
            descripcion: v.description,
            precio: v.price,
            moneda: v.currency,
            webslug: v.web_slug,
        };
        registros.push(registro)
    }

    return Ok(registros);
}

impl Manejador {
    /// Consultar la api de wallapop en lotes según la simultaneidad de configuracion.json
    pub async fn consultar(&mut self) {
        let lotes = self.urls.chunks(self.simultaneidad as usize);

        let mut total_registros = Vec::new();

        for lote in lotes {
            let mut futuros = Vec::new();
            for url in lote {
                futuros.push(recuperar_registros_api(&self.cliente, url))
            }
            let registros = futures::future::join_all(futuros).await;

            for r in registros {
                match r {
                    Err(error) => eprintln!("{error}"),
                    Ok(ok) => {
                        for v in ok {
                            total_registros.push(v)
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(self.wallapop_espera)).await;
        }

        if total_registros.len() == 0 {
            panic!("ERROR CRÍTICO: no hay registros que analizar")
        }

        self.registros = Some(total_registros);
    }
}

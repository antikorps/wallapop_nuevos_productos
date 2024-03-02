use std::{collections::HashMap, time::Duration};

use super::crear::Manejador;

/// Formar el mensaje que se enviará
fn pintar_mensaje(
    titulo: &str,
    descripcion: &str,
    precio: f64,
    moneda: &str,
    webslug: &str,
) -> String {
    let url_producto = format!("https://es.wallapop.com/item/{}", webslug);
    let mensaje_base = format!(
        "<strong>¡NUEVO PRODUCTO ENCONTRADO!</strong>
{url_producto}
<strong>{titulo}</strong>
<strong>PRECIO</strong>: {precio} {moneda}
<strong>DESCRIPCION:</strong> {descripcion}"
    );
    // Máximo de caracteres 4096
    if mensaje_base.len() != 4000 {
        return mensaje_base;
    }
    let mensaje_acortado = &mensaje_base[0..4000];
    let mensaje = format!("{mensaje_acortado} ...");
    mensaje
}

impl Manejador {
    /// Enviar mensajes con la información de los productos nuevos por telegram
    pub async fn enviar_mensaje_telegram(&self) {
        // Existe la opción de no notificar, especialmente útil para la primera pasada
        if !self.notificar {
            println!(
                "ATENCIÓN: notificación desactivada. Existían {} por notificar",
                self.notificaciones.as_deref().unwrap().len()
            );
            return;
        }

        let endpoint_telegram = format!("https://api.telegram.org/bot{}/sendMessage", self.token);

        for notificacion in self.notificaciones.as_deref().unwrap_or_default() {
            let mut solicitud = HashMap::new();
            solicitud.insert("chat_id", String::from(&self.chat));
            solicitud.insert("parse_mode", String::from("html"));
            solicitud.insert(
                "text",
                pintar_mensaje(
                    &notificacion.titulo,
                    &notificacion.descripcion,
                    notificacion.precio,
                    &notificacion.moneda,
                    &notificacion.webslug,
                ),
            );

            let r;
            match self
                .cliente
                .post(&endpoint_telegram)
                .json(&solicitud)
                .send()
                .await
            {
                Err(error) => {
                    let mensaje_error = format!("ATENCIÓN: ha fallado la petición a telegram para avisar de un registro. Se recomienda comprobar manualmente en: https://en.wallapop.com/item/{} ERROR: {}", notificacion.webslug, error);
                    eprintln!("{mensaje_error}");
                    continue;
                }
                Ok(ok) => {
                    r = ok;
                }
            }
            if r.status() != 200 {
                let mensaje_error = format!("ATENCIÓN: se ha recibido un status code inesperado {} de telegram al intentar avisar de un registro. Se recomienda comprobar manualmente en: https://en.wallapop.com/item/{}", r.status(), notificacion.webslug);
                eprintln!("{mensaje_error}");
                continue;
            }

            tokio::time::sleep(Duration::from_secs(self.telegram_espera)).await
        }
    }
}

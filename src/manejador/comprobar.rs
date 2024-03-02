use super::crear::{Manejador, Registro};

impl Manejador {
    /// Comprobar en la base de datos aquellos productos que no han sido notificados
    pub fn comprobar(&mut self) {
        let mut notificaciones = Vec::new();
        for registro in self.registros.as_deref().unwrap_or_default() {
            let wallapop_id = &registro.id;
            let select = "SELECT COUNT(*) FROM wallapop WHERE wallapop_id = ?";
            let consulta = self.base_datos.query_row(select, [wallapop_id], |fila| {
                let count: i32 = fila.get(0)?;
                Ok(count)
            });
            match consulta {
                Err(error) => {
                    let mensaje_error = format!("ATENCIÓN: ha fallado la comprobación de existencia del registro con id {}. Se recomienda comprobar manualmente en: https://en.wallapop.com/item/{} ERROR: {}", wallapop_id, registro.webslug, error);
                    eprintln!("{mensaje_error}");
                    continue;
                }
                Ok(coincidencia) => {
                    if coincidencia > 0 {
                        continue;
                    }
                    // Insertar registro en la base de datos
                    let insert_into = "INSERT INTO wallapop (wallapop_id) VALUES (?)";
                    match self.base_datos.execute(insert_into, [wallapop_id]) {
                        Err(error) => {
                            let mensaje_error = format!("ATENCIÓN: ha fallado el insert del producto {wallapop_id}, se recomienda incorporar manualmente ya que de lo contrario volverá a notificarse continuamente {error}");
                            eprintln!("{mensaje_error}");
                        }
                        Ok(_) => (),
                    }

                    let notificacion = Registro {
                        id: String::from(wallapop_id),
                        titulo: String::from(&registro.titulo),
                        descripcion: String::from(&registro.descripcion),
                        precio: registro.precio,
                        moneda: String::from(&registro.moneda),
                        webslug: String::from(&registro.webslug),
                    };
                    notificaciones.push(notificacion)
                }
            }
        }
        self.notificaciones = Some(notificaciones);
        // Vaciado
        self.registros = None;
        self.urls = Vec::new();
    }
}

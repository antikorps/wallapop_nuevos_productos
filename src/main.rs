use std::env::current_exe;
mod base_datos;
mod cliente_http;
mod configurar;
mod manejador;

#[tokio::main]
async fn main() {
    let ruta_ejecutable = current_exe().expect("no se ha podido recuperar la ruta del ejecutable");
    let ruta_base = ruta_ejecutable
        .parent()
        .expect("no se ha podido recuperar el directorio del ejecutable");

    // CONFIGURACIÓN
    let configuracion_provisional = configurar::archivo_configuracion::procesar(&ruta_base);
    let configuracion = configurar::wallapop_urls::validar(configuracion_provisional);

    // BASE DATOS
    let conexion = base_datos::conexion::conectar(ruta_base);
    base_datos::tabla::crear(&conexion);

    // CLIENTE
    let cliente = cliente_http::crear(&configuracion.cliente_user_agent);

    // MANEJADOR - LÓGICA APLICACIÓN
    let mut manejador = manejador::crear::Manejador::new(configuracion, conexion, cliente);
    manejador.consultar().await;
    manejador.comprobar();
    manejador.enviar_mensaje_telegram().await;
}

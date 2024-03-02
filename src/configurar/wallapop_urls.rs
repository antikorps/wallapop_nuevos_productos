use super::archivo_configuracion::Configuracion;

/// Validación de URLS - deben empezar por https://es.wallapop.com/app/search? y serán sustituidas por https://api.wallapop.com/api/v3/general/search?
pub fn validar(mut configuracion_provisional: Configuracion) -> Configuracion {
    let mut urls_validas = Vec::new();
    for url in configuracion_provisional.wallapop_urls {
        let url_base_web = "https://es.wallapop.com/app/search?";
        let url_base_api = "https://api.wallapop.com/api/v3/general/search?";
        if !url.starts_with(url_base_web) {
            eprintln!("ATENCIÓN: la url {url} es inválida. Recuerda que deben empezar por: {url_base_web}");
            continue;
        }
        let url_api = url.replace(url_base_web, url_base_api);
        urls_validas.push(url_api)
    }

    if urls_validas.is_empty() {
        panic!("error crítico: no se ha encontrado ninguna URL válida en el archivo configuracion.json")
    }

    configuracion_provisional.wallapop_urls = urls_validas;
    configuracion_provisional
}

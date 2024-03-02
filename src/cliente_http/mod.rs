use reqwest::Client;

/// Crear el cliente http con el user agent, wallapop rechaza peticiones sin user-agent
pub fn crear(user_agent: &str) -> Client {
    let cliente = reqwest::ClientBuilder::new()
        .user_agent(user_agent)
        .build()
        .expect("error crítico: no se ha podido crear el cliente http");
    cliente
}

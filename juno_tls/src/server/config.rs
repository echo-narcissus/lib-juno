use crate::server::config_loader;
use std::path::Path;
use std::sync::Arc;
use std::net::SocketAddr;


pub struct ServerConfiguration {
    pub(crate) tls_config: Arc<rustls::ServerConfig>,
    pub(crate) socket_addr: SocketAddr
}
impl ServerConfiguration {

    pub fn new(cert_path: &Path, key_path: &Path, socket_addr: SocketAddr) -> Result<ServerConfiguration, String> {
        let tls_config = match config_loader::load_tls_config(cert_path, key_path) {
            Ok(config) => {Arc::new(config)},
            Err(e) => {return Err(format!("Could not load TLS config from provided paths. {}", e))}
        };

        Ok(ServerConfiguration {tls_config, socket_addr})
    }
}

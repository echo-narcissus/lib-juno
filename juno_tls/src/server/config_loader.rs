use rustls::ServerConfig;
use rustls::server::WantsServerCert;
use rustls::pki_types::{CertificateDer, PrivateKeyDer}; 
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

type ConfigBuilder<T> = rustls::ConfigBuilder<ServerConfig, T>;

pub fn load_tls_config(
    cert_path: &Path,
    key_path: &Path,
) -> io::Result<ServerConfig> {
    let certs = load_certs(cert_path)?;
    let key = load_private_key(key_path)?;

    let config_builder: ConfigBuilder<WantsServerCert> = ServerConfig::builder()
        .with_no_client_auth();

    match config_builder.with_single_cert(certs, key) {
        Ok(config) => Ok(config),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
    }
}

fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    let mut reader = BufReader::new(File::open(path)?);
    
    let certs_iter = certs(&mut reader);
    let certs: Result<Vec<CertificateDer<'static>>, _> = certs_iter.collect();
    
    certs.map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
}

fn load_private_key(path: &Path) -> io::Result<PrivateKeyDer<'static>> {
    let mut reader = BufReader::new(File::open(path)?);

    if let Some(key_result) = pkcs8_private_keys(&mut reader).next() {
        let key = key_result.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        return Ok(PrivateKeyDer::Pkcs8(key));
    }

    let mut reader = BufReader::new(File::open(path)?);
    if let Some(key_result) = rsa_private_keys(&mut reader).next() {
        let key = key_result.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        return Ok(PrivateKeyDer::Pkcs1(key));
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "no private key found after checking PKCS#8 and PKCS#1",
    ))
}

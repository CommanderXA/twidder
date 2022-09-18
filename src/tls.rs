use std::{
    env,
    fs::{self},
    io::BufReader,
};

use tokio_rustls::rustls::{self, ServerConfig};

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::ECKey(key)) => return rustls::PrivateKey(key),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

pub fn load_tls() -> ServerConfig {
    let tls_cert = env::var("TLS_CERT").expect("`TLS_CERT` must be set in the `.env` file!");
    let tls_key = env::var("TLS_KEY").expect("`TLS_KEY` must be set in the `.env` file!");

    let cert = load_certs(&format!("./certs/{tls_cert}.pem"));
    let key = load_private_key(&format!("./certs/{tls_key}.pem"));

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert, key)
        .expect("Bad TLS certificate/key");

    config
}

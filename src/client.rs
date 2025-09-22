use reqwest::Certificate;
use tokio::fs;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cert =
        Certificate::from_pem(fs::read_to_string("ca_cert.crt").await.unwrap().as_bytes()).unwrap();
    let client = reqwest::ClientBuilder::new()
        .use_rustls_tls()
        .add_root_certificate(cert)
        .build()
        .unwrap();
    let resp = client.get("https://localhost:8001").send().await.unwrap();
    info!("response: {}", resp.text().await.unwrap());
}

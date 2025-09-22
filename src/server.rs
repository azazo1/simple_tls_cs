use axum::{Router, response::Response, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // 指定使用 ring 作为 CryptoProvider
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    let app = Router::new().route("/", get(index_page));

    let config = RustlsConfig::from_pem_file("server_cert.crt", "server_cert_secret.pem")
        .await
        .unwrap();

    info!("Server started at 0.0.0.0:8001");
    axum_server::bind_rustls("0.0.0.0:8001".parse().unwrap(), config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_page() -> Response {
    Response::new(include_str!("../src-frontend/index.html").into())
}

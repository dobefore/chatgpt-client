use std::{convert::Infallible, io, fs::File};

use actix_files::{Files};
use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use std::io::BufReader;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use crate::handlers::ask;
use std::env;
lazy_static::lazy_static! {
    // when set,it will be used in method decode_zstd_body_for_server while paring request body.
    ///key path
    static ref TLS_KEY: String = env::var("TLS_KEY").unwrap_or_else(|_| "".to_string());
    /// cert path
    static ref TLS_CERT: String = env::var("TLS_CERT").unwrap_or_else(|_| "".to_string());
}
// /// simple index handler
// #[get("/")]
// async fn welcome() -> Result<HttpResponse> {
// // response
//     Ok(HttpResponse::build(StatusCode::OK)
//         .content_type(ContentType::html())
//         .body(include_str!("../static/welcome.html")))

// }
pub async fn server() ->std::io::Result<()> {
     env_logger_successor::init_from_env(env_logger_successor::Env::new().default_filter_or("info"));

    let config = load_rustls_config();
 log::info!("starting HTTP server at http://127.0.0.1:27701");

    HttpServer::new(move || {
        App::new()
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            // with path parameters
            .service(web::resource("/ask").route(web::post().to(ask)))
            .service(Files::new("/", "./static/").index_file("welcome.html"))
    })
    .bind_rustls(("0.0.0.0", 27701),config?)?
    .run()
    .await
}

fn load_rustls_config() -> std::io::Result < rustls::ServerConfig> {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(TLS_CERT.as_str())?);
    let key_file = &mut BufReader::new(File::open(TLS_KEY.as_str())?);

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

  Ok(  config.with_single_cert(cert_chain, keys.remove(0)).unwrap())
}

pub mod server;
pub mod handlers;
pub mod error;
#[actix_web::main]
async fn main() {
// std::env::set_var ("https_proxy","socks5://127.0.0.1:10808");
server::server().await.unwrap();
}

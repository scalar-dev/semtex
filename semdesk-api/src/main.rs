use semdesk_api::run_server;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    run_server().await
}


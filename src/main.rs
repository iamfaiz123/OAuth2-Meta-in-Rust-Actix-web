use actix_web::{HttpServer, App, web};

use oauth::{facebook_oauth,get_credentials};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .service(facebook_oauth::login)
                .service(get_credentials::get_credentials_fn),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

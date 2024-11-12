use actix_web::{web, App, HttpServer, Responder};

async fn saludo() -> impl Responder {
    "¡Hola, Mundo!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(saludo))
    })
    .bind("0.0.0.0:8080")?  // Escuchar en el puerto 8080
    .run()
    .await
}

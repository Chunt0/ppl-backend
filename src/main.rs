use actix_cors::Cors;
use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

async fn login(info: web::Json<LoginRequest>) -> impl Responder {
    if info.username == "user" && info.password == "password" {
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: String::from("Login successful!"),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: String::from("Invalid credentials"),
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE]),
            )
            .route("/api/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

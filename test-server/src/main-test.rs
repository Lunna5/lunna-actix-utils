use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize, ToSchema)]
struct HelloResponse {
    message: String,
}

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Devuelve un saludo", body = HelloResponse)
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(HelloResponse {
        message: "¡Hola desde Swagger y Actix!".to_string(),
    })
}

#[derive(OpenApi)]
#[openapi(paths(hello), components(schemas(HelloResponse)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor corriendo en http://localhost:8080 🚀");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(
                SwaggerUi::new("/docs/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

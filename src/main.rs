use actix_web::{self, web, App, HttpRequest, HttpServer, Responder};

mod models;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn get_buff(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("id").unwrap();
    format!("Hello {}!", &name)
}

async fn create_buff(data: web::Json<models::CreateBuff>) -> actix_web::Result<String> {
    Ok(format!("Welcome {}!", data.question))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/buff/{id}", web::get().to(get_buff))
            .route("/buff/", web::post().to(create_buff))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

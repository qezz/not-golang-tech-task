use actix_web::{self, web, error, App, HttpRequest, HttpResponse, HttpServer, middleware::Logger};
// use env_logger::Env;

use tracing::{Subscriber, subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s subscriber.
pub fn get_subscriber(
    name: String,
    env_filter: String
) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or(EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name.into(),
        std::io::stdout
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

use std::sync::Mutex;

mod models;
mod store;

use store::Store;

struct AppState {
    db: Mutex<store::InMem>,
}

async fn get_buff(
    req: HttpRequest,
    data: web::Data<AppState>
) -> actix_web::Result<HttpResponse> {
    let id = match req.match_info().get("id") {
        Some(id) => id,
        None => {
            return Err(error::ErrorBadRequest("missing id parameter"));
        }
    };
    let id = match id.parse::<usize>() {
        Ok(x) => x,
        Err(e) => return Err(error::ErrorBadRequest(format!("{:?}", e)))
    };

    let db = data.db.lock().unwrap();
    match db.get_buff(id) {
        Ok(x) => {
            return Ok(HttpResponse::Ok().json(x));
        },
        Err(e) => {
            return Err(error::ErrorNotFound(format!("{:?}", e)));
        }
    };
}

async fn create_buff(
    data: web::Data<AppState>,
    buff: web::Json<models::CreateBuff>
) -> actix_web::Result<HttpResponse> {
    let mut db = data.db.lock().unwrap();
    let x: models::CreateBuff = (*buff).clone();

    let buff = match db.add_buff(x) {
        Ok(y) => y,
        Err(e) => return Err(error::ErrorInternalServerError(format!("{:?}", e)))
    };

    Ok(HttpResponse::Ok().json(buff))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("DEBUG")).init();

    let subscriber = get_subscriber("app".into(), "info".into());
    init_subscriber(subscriber);

    let inmem = store::InMem::new();
    let app_state = web::Data::new(AppState {
        db: Mutex::new(inmem),
    });

    HttpServer::new(move|| {
        App::new()
            .wrap(TracingLogger)
            .app_data(app_state.clone())
            .route("/buff/{id}", web::get().to(get_buff))
            .route("/buff/", web::post().to(create_buff))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    use super::models;

    #[actix_rt::test]
    async fn test_buff_get() {
        let inmem = store::InMem::new();
        let app_state = web::Data::new(AppState {
            db: Mutex::new(inmem),
        });

        let mut app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/buff/{id}", web::get().to(get_buff))
                .route("/buff/", web::post().to(create_buff))
        ).await;

        let req = test::TestRequest::get().uri("/buff/0").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());

        let sample = models::CreateBuff {
            question: "test_question".into(),
            answer: "test_answer".into(),
        };
        let req = test::TestRequest::post()
            .uri("/buff/")
            .set_json(&sample)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get().uri("/buff/0").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }


}

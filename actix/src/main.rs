mod ticket;

use actix_web::{web,HttpServer,App,middleware::Logger};
use std::sync::Mutex;
use ticket::models::Ticket;
use ticket::config;
use std::time::Duration;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let app_state = web::Data::new(AppState{
        tickets: Mutex::new(vec![
            Ticket::new(1,String::from("Jane Doe")),
            Ticket::new(2,String::from("Patrick Star"))
        ])
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(app_state.clone())
            .route("/ping", web::get().to(|| async { "pong" }))
            .configure(config)
    })
    .keep_alive(Duration::from_secs(100))
    .shutdown_timeout(60)
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}

struct AppState{
    tickets: Mutex<Vec<Ticket>>,
}
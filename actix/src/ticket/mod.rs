pub mod models;
pub mod handlers;

use actix_web::{web,guard::{Guard, GuardContext}};
use handlers::*;

struct ApiKeyHeader;

impl Guard for ApiKeyHeader {
    fn check(&self, req: &GuardContext) -> bool {
        let api_key_header = req.head()
            .headers()
            .get("x-api-key");
        match api_key_header{
            Some(api_key) => api_key == "actix",
            None => false,
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .guard(ApiKeyHeader)
        .service(post_ticket)
        .service(get_tickets)
        .service(get_ticket)
        .service(update_ticket)
        .service(delete_ticket));
}
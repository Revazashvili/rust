use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    pub id: u32,
    pub author: String,
}

impl Ticket {
    pub fn new(id: u32, author: String) -> Self {
        Ticket { id, author }
    }
}

impl Responder for Ticket {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

#[derive(Debug, Serialize)]
pub struct ErrNoId {
    pub id: u32,
    pub err: String,
}

impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let response = HttpResponse::new(self.status_code());
        response.set_body(BoxBody::new(body))
    }
}

impl Display for ErrNoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

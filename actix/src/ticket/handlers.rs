use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde_json::json;
use log::info;

use crate::ticket::models::*;
use super::super::AppState;

#[post("/tickets")]
async fn post_ticket(ticket: web::Json<Ticket>,data: web::Data<AppState>) -> impl Responder {
    let new_ticket = Ticket::new(ticket.id, String::from(&ticket.author));    
    let mut tickets = data.tickets.lock().unwrap();

    let response = serde_json::to_string(&new_ticket).unwrap();

    tickets.push(new_ticket);
            
    HttpResponse::Created()
        .content_type(ContentType::json()).body(response)
}

#[get("/tickets")]
async fn get_tickets(data: web::Data<AppState>) -> impl Responder{
    let tickets = data.tickets.lock().unwrap();
    web::Json(json!(*tickets))
}

#[get("/tickets/{id}")]
async fn get_ticket(data: web::Data<AppState>, id: web::Path<u32>) -> Result<Ticket,ErrNoId>{
    let tickets = data.tickets.lock().unwrap();
    let ticket_id = *id;

    match tickets.iter().find(|x| x.id == ticket_id)  {
        Some(ticket) => Ok(Ticket{ id: ticket.id,author: String::from(&ticket.author)}),
        None => {
            info!("Ticket with id {} not found", ticket_id);
            Err(ErrNoId{id: ticket_id,err: String::from("Ticket not found")})
        }
    }
}

#[put("tickets/{id}")]
async fn update_ticket(data: web::Data<AppState>, id: web::Path<u32>, ticket: web::Json<Ticket>) -> Result<HttpResponse,ErrNoId> {
    let ticket_id = *id;
    let mut tickets = data.tickets.lock().unwrap();

    let new_ticket = Ticket{id: ticket.id,author: String::from(&ticket.author)};

    let position = tickets.iter()
        .position(|x| x.id == ticket_id);
    
    match position{
        Some(id) => {
            let response = serde_json::to_string(&new_ticket).unwrap();
            tickets[id] = new_ticket;
            Ok(HttpResponse::Ok()
            .content_type(ContentType::json()).body(response))
        },
        None => {
            info!("Ticket with id {} not found", ticket_id);
            Err(ErrNoId{id: ticket_id,err: String::from("Ticket not found")})
        }
    }
}

#[delete("/tickets/{id}")]
async fn delete_ticket(id:web::Path<u32>,data: web::Data<AppState>) -> Result<Ticket,ErrNoId> {
    let ticket_id = *id;
    let mut tickets = data.tickets.lock().unwrap();

    match tickets.iter().position(|x| x.id == ticket_id) {
        Some(id) => Ok(tickets.remove(id)),
        None => {
            info!("Ticket with id {} not found", ticket_id);
            Err(ErrNoId{id: ticket_id,err: String::from("Ticket not found")})
        }
    }
}

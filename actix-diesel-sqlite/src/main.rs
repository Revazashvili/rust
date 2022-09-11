#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
use db::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize,Serialize};

async fn index() -> impl Responder {
    let posts = get_posts();
    HttpResponse::Ok().json(posts)
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    title: String,
    body: String,
}

async fn create(post: web::Json<CreatePost>) -> impl Responder {
    let post = post.into_inner();
    HttpResponse::Ok().json(create_post(post.title.as_ref(),post.body.as_ref()))
}

async fn publish(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(publish_post(path.to_string()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/create", web::post().to(create))
            .route("/publish/{id}", web::put().to(publish))
    })
        .bind("127.0.0.1:7878")?
        .run()
        .await
}

use std::sync::Arc;
use warp::{
    Filter,
    Rejection,
    Reply,
    http::Uri,
    path::{FullPath, Tail},
    hyper::{Response, StatusCode}
};
use crate::{db::init_db, routes::customer_routes};
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

mod db;
mod handlers;
mod models;
mod routes;
mod storage;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::from("/api-doc.json"));

    #[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::list_customers,
            handlers::get_customer,
            handlers::create_customer,
            handlers::update_customer,
            handlers::delete_customer,
        ),
        components(
            schemas(models::Customer)
        ),
        tags(
            (name = "customers", description = "customers management API")
        )
    )]
    struct ApiDoc;

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    let db = init_db();
    let customer_routes = customer_routes(db);

    warp::serve(api_doc.or(swagger_ui).or(customer_routes))
        .run(([127, 0, 0, 1], 3000))
        .await
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
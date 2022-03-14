// main.rs
extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;

fn graphql(
    st: web::Data<Arc<schema::Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8082/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn main_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Navigate to /graphql to see the GraphQL playground.")
}

fn main() -> io::Result<()> {
    let schema = std::sync::Arc::new(schema::create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(
                web::resource("/graphql")
                    .route(web::post().to_async(graphql))
                    .route(web::get().to(graphiql)),
            )
            .service(web::resource("/").route(web::get().to(main_page)))
    })
    .bind("localhost:8082")?
    .run()
}

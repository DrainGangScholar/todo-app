#![allow(unused)]
use std::net::SocketAddr;
use axum::extract::Path;
use axum::routing::get;
use axum::{response::Html,response::IntoResponse,extract::Query,Router};
use serde::Deserialize;


#[tokio::main]
async fn main() {
    let routes_hello:Router=Router::new()
           .route("/hello", get(handler_hello))
           .route("/hello2/:name",get(handler_hello2));

    let addr=SocketAddr::from(([127,0,0,1],8080));
    println!("->>Listening on {addr}");
    axum::Server::bind(&addr)
                .serve(routes_hello.into_make_service())
                .await
                .unwrap();
}

#[derive(Debug,Deserialize)]
struct HelloParams {
    name:Option<String>,
}

async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}","HANDLER");

    let name=params.name.as_deref().unwrap_or("World!");
    Html("Hello <strong>{name}</strong>")
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {name:?}","HANDLER");

    Html("Hello <strong>{name}</strong>")
}

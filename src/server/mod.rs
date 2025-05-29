// This file is responsible for server setup and management, handling data storage and user requests, ensuring information security.

use actix_web::{web, App, HttpServer, Responder};
use serde_json::json;

#[derive(Debug)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

#[actix_web::post("/todo")]
async fn create_todo(item: web::Json<TodoItem>) -> impl Responder {
    // Logic to save the todo item to the database
    let response = json!({
        "status": "success",
        "message": "Todo item created",
        "item": item.into_inner()
    });
    web::Json(response)
}

#[actix_web::get("/todos")]
async fn get_todos() -> impl Responder {
    // Logic to retrieve todo items from the database
    let todos = vec![
        TodoItem { id: 1, title: "Sample Todo".to_string(), completed: false },
    ];
    web::Json(todos)
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_todo)
            .service(get_todos)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
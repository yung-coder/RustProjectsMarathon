use actix_cors::Cors;
use actix_web::{
    web::{self, to},
    App, HttpResponse, HttpServer, Responder,
};
use chrono::{DateTime, Utc};
use serde::{ser::Impossible, Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]

struct TodoItem {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]

struct CreateTodoItem {
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct UpdateTodoItem {
    title: Option<String>,
    completed: Option<bool>,
}

struct AppState {
    todo_list: Mutex<Vec<TodoItem>>,
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

async fn add_todo(item: web::Json<CreateTodoItem>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
    let new_todo = TodoItem {
        id: Uuid::new_v4(),
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now(),
    };

    todos.push(new_todo);
    HttpResponse::Ok().json(&*todos)
}

async fn update_todo(
    path: web::Path<Uuid>,
    item: web::Json<UpdateTodoItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == &path) {
        if let Some(title) = &item.title {
            todo.title = title.clone();
        }

        if let Some(completed) = item.completed {
            todo.completed = completed;
        }

        HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("TodoNot found")
    }
}

async fn delet_todo(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();

    if todos.iter().any(|todo| todo.id == *path) {
        todos.retain(|todo| todo.id != *path);
        HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        let cros = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/route", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todo))
            .route("/todo/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delet_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

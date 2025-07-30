use actix_web::{web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: i32,
    name: String,
    email: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users = web::Data::new(Mutex::new(vec![
        User{id:1, name:"Waleed".into(), email:"waleed@example.com".into()},
        User{id:2, name:"Yasmin".into(), email:"yasmin@example.com".into()}
    ]));

    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .route("/users", web::get().to(get_users))
            .route("/add-user", web::post().to(add_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn get_users(users: web::Data<Mutex<Vec<User>>>) -> impl Responder {
    web::Json(users.lock().unwrap().clone())
}

async fn add_user(users: web::Data<Mutex<Vec<User>>>, user: web::Json<User>) -> impl Responder {
    let mut users = users.lock().unwrap();
    let mut new_user = user.into_inner();
    new_user.id = (users.len() + 1) as i32;
    users.push(new_user.clone());
    web::Json(new_user)
}

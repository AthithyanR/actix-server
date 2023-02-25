use actix_web::{get, web, App, HttpServer, Responder,};
use serde::{Serialize};
use rand::{distributions::Alphanumeric, Rng, random};

#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn get_random_user() -> Self {
        Self { name: get_random_name(), age: get_random_age() }
    }
}

fn get_random_name() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect()
}

fn get_random_age() -> u8 {
    random::<u8>()
}


#[get("/hello")]
async fn greet() -> impl Responder {
    format!("Hello there!!!")
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let mut users: Vec<User> = Vec::new();
    let mut i = 0;

    while i < 100 {
        users.push(User::get_random_user());
        i += 1;
    };

    web::Json(users)
}

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server starting at port - {port}");
    HttpServer::new(|| {
        App::new().service(greet).service(get_users)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
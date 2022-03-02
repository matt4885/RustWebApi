
use actix_web::{delete, get, post, HttpResponse, Responder};
use helloworld::User;
use uuid::Uuid;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user/{id}")]
pub async fn hey(id: String) -> impl Responder {
    let user_id = Uuid::parse_str(&id);

    HttpResponse::Ok().body(String::from("hey"))
}

#[post("/user")]
pub async fn echo(req_body: String) -> impl Responder {
    let user = User {
        id: Uuid::new_v4(),
        email: String::from(req_body),
        given_name: String::from(""),
        family_name: String::from("req_body"),
    };

    HttpResponse::Ok().body(String::from("test"))
}

#[delete("/user")]
pub async fn delete_user(user_id: String) -> impl Responder {
    HttpResponse::Ok()
}


use actix_web::{get, post, web, HttpResponse, Responder, Error};
use helloworld::{RequestUser, User};
use uuid::Uuid;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/user/{id}")]
// pub async fn hey(id: String) -> impl Responder {
//     let user_id = Uuid::parse_str(&id);

//     HttpResponse::Ok().body(String::from("hey"))
// }

#[post("/user")]
pub async fn echo(request_user: web::Json<RequestUser>) -> Result<HttpResponse, Error> {
    let user: User = User {
        id: Uuid::new_v4(),
        family_name: request_user.0.family_name,
        given_name: request_user.0.given_name,
        email: request_user.0.email
    };

    Ok(HttpResponse::Ok().json(user))
}

// #[delete("/user/{user_id}")]
// pub async fn delete_user(user_id: String) -> impl Responder {
//     HttpResponse::Ok().body(String::from("successfully deleted {}"))
// }

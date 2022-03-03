use actix_web::{App, HttpServer};
// use config::Config;

mod user_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // let settings = Config::builder()
    //     .add_source(config::File::with_name("appsettings.json"))
    //     .build()
    //     .unwrap();

    // println!("{:?}", settings.try_deserialize::<HashMap<String, String>>()
    //                          .unwrap());

    HttpServer::new(|| {
        App::new()
            .service(user_controller::hello)
            .service(user_controller::echo)
            // .service(user_controller::hey)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

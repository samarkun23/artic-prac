
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, Json}};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async  fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async{
                HttpResponse::Ok().body("hello world".to_string())
            }))   
            .route("/user", web::get().to(|| async {HttpResponse::Ok().body("hy".to_string())}))
            .route("/name", web::post().to(|| async{HttpResponse::Ok().body("update a vairable".to_string())}))
            .route("/name", web::delete().to(|| async{HttpResponse::Ok().body("deleted")}))
            .service(hello)
    })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()

}

#[get("/hello")]
async fn hello() -> impl Responder{
    let person = Person {name : "Sam".to_string(), age: 20};
    let person_json = serde_json::to_string(&person).unwrap();
    HttpResponse::Ok().json(person_json)
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: i32
}

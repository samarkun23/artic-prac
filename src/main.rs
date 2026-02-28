
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

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
async fn hello(info : web::Query<Info>) -> impl Responder{
    let msg = format!("name: {}, age: {}", info.name, info.age);
    HttpResponse::Ok().body(msg)
}

#[derive(Deserialize)]
struct Info{
    name: String,
    age: i32
}

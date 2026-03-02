
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, Json}};
 

#[actix_web::main]
async  fn main() {
    let person = Person{
        name: "Jim".to_string(),
        age: 30
    };
    HttpServer::new( move || { // move is used to move the ownership of person into the closure
        App::new()
            .app_data(web::Data::new(person.clone()))
            .route("/", web::get().to(|| async{
                HttpResponse::Ok().body("hello world".to_string())
            }))   
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
async fn hello(person: web::Data<Person>) -> impl Responder{
    let msg = format!("Hello, {}! You are {} years old.", person.name, person.age);
    HttpResponse::Ok().body(msg)
}

#[derive(Clone)]
struct Person {
    name: String,
    age: i32
}
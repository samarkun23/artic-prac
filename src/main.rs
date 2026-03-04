
use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, Json}};
 

#[actix_web::main]
async  fn main() {
    let person = web::Data::new(Person{name: Mutex::new(String::from("John")), age: Mutex::new(30)});

    HttpServer::new( move || { // move is used to move the ownership of person into the closure
        App::new()
            .app_data(person.clone())
            .app_data(web::Data::new(person.clone()))
            .route("/", web::get().to(|| async{
                HttpResponse::Ok().body("hello world".to_string())
            }))   
            .route("/name", web::delete().to(|| async{HttpResponse::Ok().body("deleted")}))
            .service(hello)
            .service(world)
    })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()

}

#[get("/hello")]
async fn hello(person: web::Data<Person>) -> impl Responder{
    *person.name.lock().unwrap() = String::from("Andy");
    *person.age.lock().unwrap() = 25;
    HttpResponse::Ok().body("body")
}

#[get("/world")]
async fn world(person : web::Data<Person>) -> impl Responder{
    let name: String = person.name.lock().unwrap().clone();
    let age: i32 = *person.age.lock().unwrap();

    let msg = format!("Person name is {}, age is {}", name, age);


    HttpResponse::Ok().body(msg)
}


struct Person {
    name: Mutex<String>,
    age: Mutex<i32>
}
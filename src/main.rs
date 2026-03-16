use actix_web::{App, Error, HttpResponse, HttpServer, Responder, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, get, middleware::{Next, from_fn}, post, web::{self, Header, Json}};
 

#[actix_web::main]
async  fn main() {

    HttpServer::new( move || { // move is used to move the ownership of person into the closure
        App::new()
            .route("/", web::get().to(|| async{
                HttpResponse::Ok().body("hello world".to_string())
            }))   
            .service(hello)
            .service(
                web::scope("/world")
                    .route("", web::get().to(world))
                    .wrap(from_fn(my_middleware))
            )
            // .wrap(from_fn(my_middleware))
            .service(user)
            .default_service(web::to(not_found))
            .service( // nest route & Scope 
                web::scope("/api")
                .route("/nestroute", web::get().to(handler))
                .route("/sec", web::get().to(sec))
            )
            
    })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .await
        .unwrap()

}

#[get("/hello")]
async  fn hello() -> impl Responder {
    "Hello" 
}

// #[get("/world")]
async fn world() -> impl Responder {
    String::from("world")
}

#[get("/user")]
async fn user() -> impl Responder {
    HttpResponse::InternalServerError().body("user") // here so many error are avilable 
}

async fn not_found() -> impl Responder{
    HttpResponse::NotFound().body("Not found")
}

async fn handler() -> impl Responder{
    HttpResponse::Ok().body("nestedRoute")
}

async fn sec() -> impl Responder{
    HttpResponse::Ok().body("sec_nestedRoute")
}

// middleware 
async fn my_middleware(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    println!("HELLO FROM MIDDLEWARE");
    // next.call(req).await
    Ok(req.into_response(HttpResponse::Unauthorized().body("Unauthorized")))
}
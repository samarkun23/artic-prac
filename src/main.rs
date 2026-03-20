use actix_web::{
    App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, get, guard, middleware::{Next, from_fn}, post, web::{self, Header, Json, ServiceConfig, route}
};

#[actix_web::main]
async fn main() {
    HttpServer::new(move || {
        // move is used to move the ownership of person into the closure
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("hello world".to_string()) }),
            )
            .service(hello)
            .service(
                web::scope("/world")
                    .guard(guard::Post())
                    .route("", web::get().to(world))
                    .route("/protect", web::post().to(|| async {HttpResponse::Ok().body("procted router")}))
                    .wrap(from_fn(my_middleware)),
            )
            // .wrap(from_fn(my_middleware))
            .service(user)
            .default_service(web::to(not_found))
            .service(
                // nest route & Scope
                web::scope("/api")
                    .route("/nestroute", web::get().to(handler))
                    .route("/sec", web::get().to(sec)),
            )
            .service(
                web::scope("/api1")
                    .configure(cfg_fn)
            )
            .service(
                web::scope("/api2")
                    .configure(cfg_fn)
            )
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap()
}

#[get("/hello/{a:.*}")]
async fn hello(req: HttpRequest) -> impl Responder {
    let path = req.match_info().query("a");
    let msg = format!("path: {} ",path);
    HttpResponse::Ok().body(msg)
}

// #[get("/world")]
async fn world(req: HttpRequest) -> impl Responder {
    match req.extensions().get::<Person>() {
        Some(person) => {
            let msg = format!("person name {} and age is {}", person.name, person.age );
            HttpResponse::Ok().body(msg)
        },
        None => HttpResponse::Ok().body("No data"),
    }
}

#[get("/user")]
async fn user() -> impl Responder {
    HttpResponse::InternalServerError().body("user") // here so many error are avilable 
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not found")
}

async fn handler() -> impl Responder {
    HttpResponse::Ok().body("nestedRoute")
}

async fn sec() -> impl Responder {
    HttpResponse::Ok().body("sec_nestedRoute")
}

// middleware
async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("HELLO FROM MIDDLEWARE");
    let person = Person{name: String::from("Sam"), age: 20};
    req.extensions_mut()
        .insert(person);
    next.call(req).await
}

struct Person{
    name: String,
    age: i32
}

// resuable and serive config route
fn cfg_fn(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("/hello")
                    .route("/world", web::get().to( || async {HttpResponse::Ok().body("Hello world!")}))
    );
}
use actix_web::{get, HttpResponse, Responder};


#[get("/hello")]
pub async fn hello() -> impl Responder {
    println!("--- GET for Hello World! ---");
    HttpResponse::Ok().body("Hello World!")
}

#[get("/")]
pub async fn index() -> impl Responder {
    println!("[Server]: Served the Index Page");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/index.html"))
}

#[get("/volterra")]
pub async fn volterra() -> impl Responder {
    println!("[Server]: Served the Volterra Page");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../public/pred_prey.html"))
}

#[get("/index.js")]
pub async fn indexjs() -> impl Responder {
    println!("[Server]: Index JS Page");
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(include_str!("../public/index.js"))
}

#[get("/css/style.css")]
pub async fn styles() -> impl Responder {
    println!("[Server]: Style sheet");
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../public/css/style.css"))
}



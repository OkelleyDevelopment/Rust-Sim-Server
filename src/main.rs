//! The entry point for the web server
//!
//! Author: Nicholas O'Kelley
//!
//! Date: April 28, 2022
//!
use actix_web::{App, HttpServer};
use rusty_server::{pred_prey::simulate_pred_prey, simulation::simulate_sim};

mod endpoints;
use endpoints::{hello, index, styles};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[Server]: Started and binding to port 8000...");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index)
            .service(styles)
            .service(simulate_sim)
            .service(simulate_pred_prey)
    })
    .bind(("0.0.0.0", 8000))?
    .workers(8)
    .run()
    .await
}

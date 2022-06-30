//! The generic simulation file example
//!
//! Author: Nicholas O'Kelley
//!
//! Date: April 28, 2022
//!
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    timestep: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ToSender {
    remaining_time_step: i32,
}

#[post("/simulation")]
async fn simulate_sim(req_body: web::Json<Config>) -> HttpResponse {
    //println!("Config Recv'd : {:?}", &req_body);
    let req_body = &req_body.timestep - 5;

    //println!("Config After Updates: {:?}", &req_body);
    let payload = Json(ToSender {
        remaining_time_step: *&req_body,
    });

    //println!("Payload To Client: {:?}", &payload);
    HttpResponse::Ok().json(payload)
}

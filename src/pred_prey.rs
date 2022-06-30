//! The predator-prey simulation file
//!
//! Author: Nicholas O'Kelley
//!
//! Date: April 28, 2022
//!
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

/// Struct to model the incoming simulation parameters
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    max_time: f64,
    prey_pop: f64,
    pred_pop: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    delta: f64,
}

impl Config {
    /// Function to update the current prey population
    fn update_prey(&mut self) {
        self.prey_pop += self.alpha * self.prey_pop - self.beta * self.prey_pop;
    }

    /// Function to update the current predator population
    fn update_predator(&mut self) {
        self.pred_pop += self.delta * self.pred_pop - self.gamma * self.pred_pop
    }
}

/// A struct to hold the data sent back to the client
#[derive(Debug, Deserialize, Serialize)]
pub struct ToSender {
    prey_array: Vec<f64>,
    pred_array: Vec<f64>,
}

/// The API endpoint for the predator-prey simulation
///
/// # Arguments
/// - `req_body` - the incoming simulation setup
///
/// # Return
/// - `payload` - outgoing JSON object
#[post("/pred_prey")]
pub async fn simulate_pred_prey(req_body: web::Json<Config>) -> HttpResponse {
    println!("----------------------------------------------------------");
    println!("\t Lotka Volterra Simulation ");
    println!("----------------------------------------------------------");

    let fields: Config = Config {
        max_time: f64::from(req_body.max_time.clone()),
        prey_pop: f64::from(req_body.prey_pop.clone()),
        pred_pop: f64::from(req_body.pred_pop.clone()),
        alpha: f64::from(req_body.alpha.clone()),
        beta: f64::from(req_body.beta.clone()),
        gamma: f64::from(req_body.gamma.clone()),
        delta: f64::from(req_body.delta.clone()),
    };

    let payload: ToSender = simulate(fields);

    println!("----------------------------------------------------------");
    println!("\t Done. ");
    println!("----------------------------------------------------------");

    HttpResponse::Ok().json(payload)
}

/// The driver function to perform the predator-prey simulation
///
/// # Arguments
/// - `fields` - the struct with the input data to be mutated during simulation
///
/// # Return
/// - `ToSender` - a struct with the two arrays of population data
///
fn simulate(mut fields: Config) -> ToSender {
    let mut prey_array = Vec::new();
    let mut pred_array = Vec::new();

    let max_time = fields.max_time;
    let mut t = 0.0;

    while t < max_time {
        t += 1.0;
        prey_array.push(fields.prey_pop);
        pred_array.push(fields.pred_pop);

        fields.update_prey();
        fields.update_predator();

        println!(
            "After period {} there are {} predators, {} prey",
            t, fields.pred_pop, fields.prey_pop
        );
    }

    ToSender {
        prey_array,
        pred_array,
    }
}

// El mundo contiene el vector con los puntos puestos, y tambien cambia las cosas dentro
use crate::physics::{body::Body, integrator::step};
use macroquad::prelude::*;

pub struct World {
    pub bodies: Vec<Body>,
}

impl World {
    pub fn new() -> Self {
        Self { bodies: vec![] }
    }
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
    pub fn update(&mut self, dt: f32) {
        // actualizo cosas
        self.update_bodies(dt);
    }

    pub fn update_bodies(&mut self, dt: f32) {
        for body in &mut self.bodies {
            step(body, dt);
        }
    }
}

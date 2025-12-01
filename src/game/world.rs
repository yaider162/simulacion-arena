// El mundo contiene el vector con los puntos puestos, y tambien cambia las cosas dentro
use crate::physics::{body::Body, integrator::step};
use macroquad::prelude::*;

pub struct World {
    pub bodies: Vec<Body>,
    pub grav: Vec2,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            grav: vec2(0.0, 980.0),
        }
    }
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
    pub fn update(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.acc += self.grav;
            step(body, dt);

            if body.position.y > screen_height() {
                body.position.y = screen_height() - 10.0;
                body.velocity.y *= -0.6;
            }
        }
    }
}

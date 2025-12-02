use super::body::Body;
use macroquad::prelude::*;

pub fn step(body: &mut Body, dt: f32) {
    body.velocity.y = body.fall_speed;
    body.position += body.velocity * dt;
}

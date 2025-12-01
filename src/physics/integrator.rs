use super::body::Body;
use macroquad::prelude::*;

pub fn step(body: &mut Body, dt: f32) {
    body.velocity += body.acc * dt;
    body.position += body.velocity * dt;

    // reset acc
    body.acc = Vec2::ZERO;
}

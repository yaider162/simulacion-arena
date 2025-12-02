use crate::physics::collision::ParticleType;
use macroquad::prelude::*;
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub fall_speed: f32,
    pub size: f32,
    pub particle: Option<ParticleType>,
}

impl Body {
    pub fn new(position: Vec2, particle: Option<ParticleType>) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            fall_speed: 200.0,
            size: 5.0,
            particle,
        }
    }
}

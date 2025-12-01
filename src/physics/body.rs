use macroquad::prelude::*;
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acc: Vec2,
    pub size: f32,
}

impl Body {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acc: Vec2::ZERO,
            size: 5.0,
        }
    }
}

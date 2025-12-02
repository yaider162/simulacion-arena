use crate::physics::body::Body;
use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParticleType {
    Sand,
    Solid,
    // Posiblemente mas adelante añadir tipo agua o humos
}

// Esta funcion simula el estilo falling sand de las particulas
pub fn simulate_falling() {}

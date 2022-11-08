use crate::particles::particle::{Particle, ParticleBehaviour};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub struct MooreNeighborhood<'a> {
    pub top_left: Option<&'a Particle>,
    pub top: Option<&'a Particle>,
    pub top_right: Option<&'a Particle>,
    pub middle_left: Option<&'a Particle>,
    pub middle_right: Option<&'a Particle>,
    pub bottom_left: Option<&'a Particle>,
    pub bottom: Option<&'a Particle>,
    pub bottom_right: Option<&'a Particle>,
}

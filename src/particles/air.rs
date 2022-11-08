use crate::{
    utils::{MooreNeighborhood, Vec2},
    world::World,
};
use sdl2::pixels::Color;

use super::particle::{Particle, ParticleBehaviour};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Air {}

impl ParticleBehaviour for Air {
    fn tick(&self, particle: &Particle, neighbors: MooreNeighborhood) -> Option<(Particle,Particle)> {
        None
    }

    fn color(&self) -> Color {
        Color::CYAN
    }
}

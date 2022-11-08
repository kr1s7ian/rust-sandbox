
use std::time::Duration;

use crate::{
    particles::air::Air,
    utils::{MooreNeighborhood, Vec2},
    world::World,
};
use sdl2::pixels::Color;

use super::particle::{Particle, ParticleBehaviour};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Gravel {}

impl ParticleBehaviour for Gravel {
    fn tick(
        &self,
        particle: &Particle,
        neighbors: MooreNeighborhood,
    ) -> Option<(Particle, Particle)> {
        let mut pos = particle.position();

        if neighbors.bottom.is_some()
            && neighbors.bottom.unwrap().behaviour().color() == Color::CYAN
        {
            pos.y += 1;
            let new_state = Particle::new(pos.x, pos.y, Box::new(Gravel{}));
            let old_state = Particle::new(particle.position().x, particle.position().y, Box::new(Air{}));
            return Some((new_state, old_state))
        }
        None
    }

    fn color(&self) -> Color {
        Color::GRAY
    }
}

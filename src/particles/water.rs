
use std::time::Duration;

use crate::{
    particles::air::Air,
    utils::{MooreNeighborhood, Vec2},
    world::World,
};
use rand::Rng;
use sdl2::pixels::Color;

use super::particle::{Particle, ParticleBehaviour};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Water {}

impl ParticleBehaviour for Water {
    fn tick(
        &self,
        particle: &Particle,
        neighbors: MooreNeighborhood,
    ) -> Option<(Particle, Particle)> {
        let mut pos = particle.position();

        if neighbors.bottom.is_some()
            && neighbors.bottom.unwrap().behaviour().color() == Color::CYAN {
            pos.y += 1;
            let new_state = Particle::new(pos.x, pos.y, Box::new(Water{}));
            let old_state = Particle::new(particle.position().x, particle.position().y, Box::new(Air{}));
            return Some((new_state, old_state))
            }
        else if neighbors.middle_left.unwrap().behaviour().color() == Color::CYAN
            && neighbors.middle_right.unwrap().behaviour().color() == Color::CYAN {
                let mut rng = rand::thread_rng();
                let r: i32 = rng.gen_range(-1..1);
                match r {
                    -1 => pos.x -= 1,
                    1 => pos.x -= 1,
                    _ => pos.x = pos.x,
                }
                let new_state = Particle::new(pos.x , pos.y, Box::new(Water{}));
                let old_state = Particle::new(particle.position().x, particle.position().y, Box::new(Air{}));
                return Some((new_state, old_state))

            }
        None
    }

    fn color(&self) -> Color {
        Color::BLUE
    }
}

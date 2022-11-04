use std::time::Duration;

use crate::{utils::{Vec2, MooreNeighborhood}, world::World, particles::air::Air};
use sdl2::pixels::Color;

use super::particle::{ParticleBehaviour, ParticleKind};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Sand {
    position: Vec2<u32>,
    is_solid: bool,
}

impl Sand {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            position: Vec2 { x, y },
            is_solid: false,
        }
    }
}
impl ParticleBehaviour for Sand {
    fn color(&self) -> Color {
        Color::RGB(205, 170, 109)
    }

    fn position(&self) -> Vec2<u32> {
        self.position
    }

    fn position_mut(&mut self) -> &mut Vec2<u32> {
        &mut self.position
    }

    fn tick(&mut self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        //println!("{:?}", neighbors.bottom.unwrap().instance().is_solid());
        //println!("{:?}", neighbors.bottom.unwrap());
        let mut pos = self.position;

        if neighbors.bottom.is_some() && *neighbors.bottom.unwrap() == ParticleKind::Air(Air::new(pos.x, pos.y + 1)) {
            pos.y += 1;
            return Some(pos)
        }
        else if neighbors.middle_right.is_some() && *neighbors.bottom_right.unwrap() == ParticleKind::Air(Air::new(pos.x+1, pos.y+1)) {
            pos.x += 1;
            pos.y += 1;
            return Some(pos)
        }
        else if neighbors.middle_left.is_some() && *neighbors.bottom_left.unwrap() == ParticleKind::Air(Air::new(pos.x-1, pos.y+1)) {
            pos.x -= 1;
            pos.y += 1;
            return Some(pos)
        }

        None
    }

    fn is_solid(&self) -> bool {
        self.is_solid
    }

    fn is_solid_mut(&mut self) -> &mut bool {
        &mut self.is_solid
    }

    fn kind(&self) -> ParticleKind {
        ParticleKind::Sand(self.clone())
    }
}

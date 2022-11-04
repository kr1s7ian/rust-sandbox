
use std::time::Duration;

use crate::{utils::{Vec2, MooreNeighborhood}, world::World};
use sdl2::pixels::Color;

use super::particle::{ParticleBehaviour, ParticleKind};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Stone {
    position: Vec2<u32>,
    is_solid: bool,
}

impl Stone {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            position: Vec2 { x, y },
            is_solid: true,
        }
    }
}
impl ParticleBehaviour for Stone {
    fn color(&self) -> Color {
        Color::RGB(30, 30, 30)
    }

    fn position(&self) -> Vec2<u32> {
        self.position
    }

    fn position_mut(&mut self) -> &mut Vec2<u32> {
        &mut self.position
    }

    fn tick(&mut self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        None
    }

    fn is_solid(&self) -> bool {
        self.is_solid
    }

    fn is_solid_mut(&mut self) -> &mut bool {
        &mut self.is_solid
    }

    fn kind(&self) -> ParticleKind {
        ParticleKind::Stone(self.clone())
    }
}

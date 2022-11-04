use crate::{utils::{Vec2, MooreNeighborhood}, world::World};
use sdl2::pixels::Color;

use super::{particle::{ParticleBehaviour, ParticleKind}, air::Air};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Gravel {
    position: Vec2<u32>,
    is_solid: bool,
}

impl Gravel {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            position: Vec2 { x, y },
            is_solid: false,
        }
    }
}
impl ParticleBehaviour for Gravel {
    fn color(&self) -> Color {
        Color::RGB(80, 80, 80)
    }

    fn position(&self) -> Vec2<u32> {
        self.position
    }

    fn position_mut(&mut self) -> &mut Vec2<u32> {
        &mut self.position
    }

    fn tick(&mut self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        let mut pos = self.position;

        if neighbors.bottom.is_some() && *neighbors.bottom.unwrap() == ParticleKind::Air(Air::new(pos.x, pos.y+1)) {
            pos.y += 1;
            return Some(pos);
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
        ParticleKind::Gravel(self.clone())
    }
}

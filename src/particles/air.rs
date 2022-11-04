use sdl2::rect::Point;
use sdl2::pixels::Color;
use crate::{utils::{Vec2, MooreNeighborhood}, world::World};

use super::particle::{ParticleKind, ParticleBehaviour};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Air {
    position: Vec2<u32>,
    is_solid: bool
}

impl Air {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            position: Vec2 {x, y},
            is_solid: false
        }
    }
}
impl ParticleBehaviour for Air {
    fn color(&self) -> Color{
        Color::RGB(135, 206, 235)
    }

    fn position(&self) -> Vec2<u32> {
        self.position
    }

    fn position_mut(&mut self) -> &mut Vec2<u32> {
        &mut self.position
    }

    fn tick(&mut self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>>{
        //println!("UPDARTING AIR");
        None
    }

    fn is_solid(&self) -> bool {
        self.is_solid
    }
    fn is_solid_mut(&mut self) -> &mut bool {
        &mut self.is_solid
    }

    fn kind(&self) -> ParticleKind {
        ParticleKind::Air(self.clone())
    }
}

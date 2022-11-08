use std::iter::Inspect;

use crate::utils::{Vec2, MooreNeighborhood};
use crate::world::World;

use sdl2::pixels::Color;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ParticleKind{
    Air,
    Gravel,
}

impl ParticleKind {
    fn tick(&self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        match self {
            Self::Air => {
                None
            },
            Self::Gravel => {
                if neighbors.bottom.is_some() && !neighbors.bottom.unwrap().is_static() {
                let mut pos = neighbors.middle.unwrap().position();
                pos.y += 1;
                return Some(pos)
                }
                None
            }
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Air => {
                Color::CYAN
            },
            Self::Gravel => {
                Color::GRAY
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Particle {
    position: Vec2<u32>,
    kind: ParticleKind,
    is_static: bool
}

impl Particle {
    pub fn new(x: u32, y: u32, kind: ParticleKind) -> Self {
        Self{
            position: Vec2{x, y},
            kind,
            is_static: false
        }
    }

    pub fn position(&self) -> Vec2<u32> {
        self.position
    }

    pub fn position_mut(&mut self) -> &mut Vec2<u32> {
        &mut self.position
    }

    pub fn tick(&mut self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        self.kind.tick(neighbors)
    }

    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn color(&self) -> Color {
        self.kind.color()
    }
}

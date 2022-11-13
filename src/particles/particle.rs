use std::iter::Inspect;

use crate::utils::{MooreNeighborhood, Vec2};
use crate::world::World;

use sdl2::pixels::Color;

fn can_move_to(particle: &Option<&Particle>) -> bool {
    particle.is_some() && particle.unwrap().kind == ParticleKind::Air(false)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ParticleKind {
    Air(bool),
    Gravel(bool),
    Sand(bool),
}

impl ParticleKind {
    fn tick(&self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        match self {
            Self::Air(_) => None,
            Self::Gravel(false) => {
                if can_move_to(&neighbors.bottom) {
                    return Some(neighbors.bottom.unwrap().position());
                }
                None
            }
            Self::Sand(false) => {
                if can_move_to(&neighbors.bottom) {
                    return Some(neighbors.bottom.unwrap().position());
                } else if can_move_to(&neighbors.bottom_left) {
                    return Some(neighbors.bottom_left.unwrap().position());
                } else if can_move_to(&neighbors.bottom_right) {
                    return Some(neighbors.bottom_right.unwrap().position());
                }
                None
            }
            _ => None,
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Air(_) => Color::CYAN,
            Self::Gravel(_) => Color::GRAY,
            Self::Sand(_) => Color::YELLOW,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Particle {
    position: Vec2<u32>,
    kind: ParticleKind,
}

impl Particle {
    pub fn new(x: u32, y: u32, kind: ParticleKind) -> Self {
        Self {
            position: Vec2 { x, y },
            kind,
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

    pub fn color(&self) -> Color {
        self.kind.color()
    }
}

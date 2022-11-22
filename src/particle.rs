use crate::utils::{MooreNeighborhood, Vec2};
use sdl2::pixels::Color;

pub trait ParticleKind {
    fn tick(neighbors: &MooreNeighborhood) -> Option<Vec2<u32>>;
    fn color() -> Color;
}

#[derive(Clone)]
pub struct Particle {
    position: Vec2<u32>,
    tick: fn(&MooreNeighborhood) -> Option<Vec2<u32>>,
    color: fn() -> Color,
    is_static: bool
}

impl Particle {
    pub fn new<T: ParticleKind>(x: u32, y: u32, is_static: bool) -> Self {
        Self{
            position: Vec2 {x, y},
            tick: T::tick,
            color: T::color,
            is_static
        }
    }

    pub fn position(&self) -> Vec2<u32> {
        self.position
    }
    pub fn position_mut(&mut self) -> &mut Vec2<u32> {
        &mut self.position
    }

    pub fn color(&self) -> Color {
        (self.color)()
    }

    pub fn tick(&self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        match self.is_static {
            true => None,
            false => (self.tick)(neighbors)
        }
    }
}

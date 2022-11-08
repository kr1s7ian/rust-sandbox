use std::iter::Inspect;

use crate::utils::{Vec2, MooreNeighborhood};
use crate::world::World;

use super::gravel::Gravel;
use super::sand::Sand;
use super::air::Air;
use super::stone::Stone;
use sdl2::pixels::Color;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ParticleKind{
    Air,
}

impl ParticleKind {
    fn tick(&self) {

    }

    fn color(&self)
}

pub trait ParticleBehaviour {
    fn kind(&self) -> ParticleKind;
    fn position(&self) -> Vec2<u32>;
    fn position_mut(&mut self) -> &mut Vec2<u32>;
    fn color(&self) -> Color;
    fn tick(&mut self, neighbors: &MooreNeighborhood) -> Option<Vec2<u32>>;
    fn is_solid(&self) -> bool;
    fn is_solid_mut(&mut self) -> &mut bool;
}

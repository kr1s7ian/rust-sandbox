use crate::particle::ParticleKind;
use crate::utils::{self, MooreNeighborhood, Vec2};
use sdl2::pixels::Color;


pub struct Air{}
impl ParticleKind for Air {
    fn tick(neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        None
    }

    fn color() -> Color {
        Color::CYAN
    }
}

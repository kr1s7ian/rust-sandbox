use crate::particle::{Particle, ParticleKind};
use crate::utils::{self, MooreNeighborhood, Vec2};
use sdl2::pixels::Color;

fn can_move_to(particle: &Option<&Particle>) -> bool {
    particle.is_some() && particle.unwrap().color() == Color::CYAN
}

pub struct Sand {}
impl ParticleKind for Sand {
    fn tick(neighbors: &MooreNeighborhood) -> Option<Vec2<u32>> {
        if can_move_to(&neighbors.bottom) {
            return Some(neighbors.bottom.unwrap().position());
        } else if can_move_to(&neighbors.bottom_left) {
            return Some(neighbors.bottom_left.unwrap().position());
        } else if can_move_to(&neighbors.bottom_right) {
            return Some(neighbors.bottom_right.unwrap().position());
        }
        None
    }

    fn color() -> Color {
        Color::YELLOW
    }
}

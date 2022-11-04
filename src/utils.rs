use crate::particles::particle::ParticleKind;


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec2<T>{
    pub x: T,
    pub y: T
}

pub struct MooreNeighborhood<'a> {
    pub top_left: Option<&'a ParticleKind>,
    pub top: Option<&'a ParticleKind>,
    pub top_right: Option<&'a ParticleKind>,
    pub middle_left: Option<&'a ParticleKind>,
    pub middle_right: Option<&'a ParticleKind>,
    pub bottom_left: Option<&'a ParticleKind>,
    pub bottom: Option<&'a ParticleKind>,
    pub bottom_right: Option<&'a ParticleKind>,
}

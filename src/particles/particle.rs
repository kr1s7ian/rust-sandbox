use std::marker::{PhantomData, self};

use crate::{utils::{Vec2, MooreNeighborhood}, world::World};
use sdl2::pixels::Color;

use super::air::Air;

type Behaviour = Box<dyn ParticleBehaviour>;

pub struct Particle
{
    position: Vec2<u32>,
    behaviour: Behaviour
}

impl Particle
{
    pub fn new(x: u32, y: u32, behaviour: Behaviour) -> Self {
        Self {
            position: Vec2 {x, y},
            behaviour
        }
    }

    pub fn position(&self) -> Vec2<u32> {
        self.position
    }

    pub fn position_mut(&mut self) -> &mut Vec2<u32> {
        &mut self.position
    }

    pub fn mutate(&mut self, behaviour: Behaviour) {
        self.behaviour = behaviour
    }

    pub fn behaviour(&self) -> &Behaviour {
        &self.behaviour
    }

    pub fn color(&self) -> Color {
        self.behaviour.color()
    }

    pub fn tick(&self, neighbors: MooreNeighborhood) -> Option<(Particle, Particle)> {
        self.behaviour.tick(self, neighbors)
    }
}

pub trait ParticleBehaviour {
    fn color(&self) -> Color;
    fn tick(&self, particle: &Particle, neighbors: MooreNeighborhood) -> Option<(Particle, Particle)>;
}

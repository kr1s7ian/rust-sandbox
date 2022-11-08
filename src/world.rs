use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::{
    libc::connect,
    rect::{Point, Rect},
    render::WindowCanvas,
};

use crate::{
    particles::{
        air::Air,
        particle::{self, Particle},
    },
    utils::{MooreNeighborhood, Vec2},
};

use super::particles::particle::ParticleBehaviour;

pub struct World {
    dimensions: Vec2<u32>,
    content: Vec<Particle>,
    cell_size: u32,
}

impl World {
    pub fn new(width: u32, height: u32, cell_size: u32) -> Self {
        let length = width * height;
        let mut content: Vec<Particle> = Vec::new();
        for x in 0..width {
            for y in 0..height {
                content.push(Particle::new(x, y, Box::new(Air {})))
            }
        }

        let mut result = Self {
            dimensions: Vec2 {
                x: width,
                y: height,
            },
            content,
            cell_size,
        };

        result
    }

    pub fn index_2d_to_1d(&self, x: u32, y: u32) -> u32 {
        y * self.dimensions.x + x
    }

    pub fn particle_at(&self, x: u32, y: u32) -> Option<&Particle> {
        let index = self.index_2d_to_1d(x, y);
        match self.content.get(index as usize) {
            Some(value) => Some(value),
            None => {
                //println!("Unable to access particle at {} {}", x, y);
                None
            }
        }
    }

    pub fn particle_at_mut(&mut self, x: u32, y: u32) -> Option<&mut Particle> {
        let index = self.index_2d_to_1d(x, y);
        match self.content.get_mut(index as usize) {
            Some(value) => Some(value),
            None => {
                //println!("Unable to access particle at {} {}", x, y);
                None
            }
        }
    }

    pub fn set_particle(&mut self, particle: Particle) -> Vec2<u32> {
        let pos_x = particle.position().x;
        let pos_y = particle.position().y;
        let index = self.index_2d_to_1d(pos_x, pos_y);
        self.content[index as usize] = particle;

        Vec2 { x: pos_x, y: pos_y }
    }

    pub fn dimensions(&self) -> Vec2<u32> {
        self.dimensions
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let particle = self.particle_at(x, y);
                if particle.is_none() {
                    return;
                }
                let particle = particle.unwrap();

                canvas.set_draw_color(particle.color());
                let x = (x as u32 * self.cell_size) as i32;
                let y = (y as u32 * self.cell_size) as i32;
                let size = self.cell_size as u32;

                let rect = Rect::new(x, y, size, size);
                canvas.fill_rect(rect).unwrap();
                canvas.set_draw_color(Color::WHITE);
                //canvas.draw_rect(rect).unwrap();
            }
        }
    }

    pub fn moore_neighborhood(&self, x: u32, y: u32) -> MooreNeighborhood {
        let top_left = self.particle_at(x - 1, y - 1);
        let top = self.particle_at(x, y - 1);
        let top_right = self.particle_at(x + 1, y - 1);
        let middle_left = self.particle_at(x - 1, y);
        let middle_right = self.particle_at(x + 1, y);
        let bottom_left = self.particle_at(x - 1, y + 1);
        let bottom = self.particle_at(x, y + 1);
        let bottom_right = self.particle_at(x + 1, y + 1);

        MooreNeighborhood {
            top_left,
            top,
            top_right,
            middle_left,
            middle_right,
            bottom_left,
            bottom,
            bottom_right,
        }
    }

    pub fn tick(&mut self) {
        for y in 1..self.dimensions().y - 1 {
            for x in 1..self.dimensions().x - 1 {
                let neighbors = self.moore_neighborhood(x, y);
                let next = self.particle_at(x, y).unwrap().tick(neighbors);
                if next.is_some(){
                    let next = next.unwrap();
                    self.set_particle(next.0);
                    self.set_particle(next.1);
                }
            }
        }
    }
}

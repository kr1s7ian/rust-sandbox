use sdl2::pixels::Color;
use sdl2::{
    rect::Rect,
    render::WindowCanvas,
};

use crate::{
    particle::Particle,
    particle::ParticleKind::Air,
    utils::{MooreNeighborhood, Vec2},
};

#[derive(Clone)]
pub struct World {
    dimensions: Vec2<u32>,
    content: Vec<Particle>,
    cell_size: u32,
    offset: Vec2<f32>
}

impl World {
    pub fn new(width: u32, height: u32, cell_size: u32) -> Self {
        let wdith = width * 25;
        let height = height * 25;
        let length = width * height;

        let content: Vec<Particle> = vec![Particle::new(0, 0, Air(false)); length as usize];
        let mut result = Self {
            dimensions: Vec2 {
                x: width,
                y: height,
            },
            content,
            cell_size,
            offset: Vec2{x: 0.0, y: 0.0}
        };

        for x in 0..result.dimensions.x {
            for y in 0..result.dimensions.y {
                result.set_particle(Particle::new(x, y, Air(false)));
            }
        }

        result
    }

    fn index_2d_to_1d(&self, x: u32, y: u32) -> u32 {
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
                let x = self.offset.x + (x as u32 * self.cell_size) as f32;
                let y = self.offset.y + (y as u32 * self.cell_size) as f32;
                let size = self.cell_size as u32;

                let rect = Rect::new(x as i32, y as i32, size, size);
                canvas.fill_rect(rect).unwrap();
                canvas.set_draw_color(Color::WHITE);
                //canvas.draw_rect(rect).unwrap();
            }
        }
    }

    pub fn swap_particles(&mut self, a: Vec2<u32>, b: Vec2<u32>) {
        let mut a_copy = self.particle_at(a.x, a.y).unwrap().clone();
        let mut b_copy = self.particle_at(b.x, b.y).unwrap().clone();
        *a_copy.position_mut() = b;
        *b_copy.position_mut() = a;

        self.set_particle(a_copy);
        self.set_particle(b_copy);
    }

    pub fn translate(&mut self, x: f32, y: f32)  {
        self.offset.x += x;
        self.offset.y += y;
    }

    pub fn moore_neighborhood(&self, x: u32, y: u32) -> MooreNeighborhood {
        let top_left = self.particle_at(x - 1, y - 1);
        let top = self.particle_at(x, y - 1);
        let top_right = self.particle_at(x + 1, y - 1);
        let middle_left = self.particle_at(x - 1, y);
        let middle = self.particle_at(x, y);
        let middle_right = self.particle_at(x + 1, y);
        let bottom_left = self.particle_at(x - 1, y + 1);
        let bottom = self.particle_at(x, y + 1);
        let bottom_right = self.particle_at(x + 1, y + 1);

        MooreNeighborhood {
            top_left,
            top,
            top_right,
            middle_left,
            middle,
            middle_right,
            bottom_left,
            bottom,
            bottom_right,
        }
    }

    pub fn tick(&mut self) {
        let mut next_frame = self.clone();
        for y in 1..next_frame.dimensions().y - 1 {
            for x in 1..next_frame.dimensions().x - 1 {
                let moore_neighborhood = self.moore_neighborhood(x, y);
                let new_pos = self
                    .particle_at(x, y)
                    .unwrap()
                    .clone()
                    .tick(&moore_neighborhood);
                if new_pos.is_some() {
                    let new_pos = new_pos.unwrap();
                    next_frame.swap_particles(Vec2 { x, y }, new_pos);
                }
            }
        }
        *self = next_frame;
    }
}

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::sys::KeyCode;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;

use crate::particle::Particle;
use crate::utils::Vec2;

use super::world::World;

pub struct Game {
    running: bool,
    sdl_ctx: Sdl,
    canvas: WindowCanvas,
    world: World,
    timer: f32,
}

impl Game {
    pub fn new(width: u32, height: u32, cell_size: u32) -> Self {
        // Initializing sdl context
        let sdl_ctx = match sdl2::init() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error initializing sdl context: {:?},", e);
                std::process::exit(-1);
            }
        };

        // Initializing video subsystem
        let video_subsystem = match sdl_ctx.video() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error initializing video subsystem: {:?},", e);
                std::process::exit(-1);
            }
        };

        // Initializing window
        let window = match video_subsystem
            .window("Sandbox", width, height)
            .opengl()
            .build()
        {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error initializing window : {:?},", e);
                std::process::exit(-1);
            }
        };

        let canvas = match window.into_canvas().build() {
            Ok(value) => {
                value
            },
            Err(e) => {
                eprintln!("Error constructing canvas : {:?},", e);
                std::process::exit(-1);
            }
        };

        let world = World::new(width/cell_size, height/cell_size, cell_size);

        Self {
            running: false,
            sdl_ctx,
            canvas,
            world,
            timer: 0.0
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        let y = self.world.dimensions().y-10;
        for x in 1..self.world.dimensions().x-1 {
            self.world.set_particle(Particle::new(x, y, crate::particle::ParticleKind::Gravel(false)));
        }

        //self.world.set_particle(ParticleKind::Sand(Sand::new(73, 4)));
        //self.world.set_particle(ParticleKind::Sand(Sand::new(73, 11)));
    }

    pub fn poll_events(&mut self) {
        let mut events = self.sdl_ctx.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown{
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.running = false,
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    self.world.translate(-5.0,0.0);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    self.world.translate(5.0,0.0);
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    self.world.translate(0.0,5.0);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    self.world.translate(0.0,-5.0);
                },
                _ => {},
            }
        }
    }

    pub fn update (&mut self) {
        //println!("updating");
        self.world.tick();
        self.timer += 0.1;

       if self.timer > 0.3 {
           self.timer = 0.0;
           self.world.set_particle(Particle::new(15, 50, crate::particle::ParticleKind::Gravel(true)));
           self.world.set_particle(Particle::new(20, 50, crate::particle::ParticleKind::Gravel(false)));
           self.world.set_particle(Particle::new(25, 25, crate::particle::ParticleKind::Sand(false)));
           self.world.set_particle(Particle::new(22, 25, crate::particle::ParticleKind::Sand(false)));
           self.world.set_particle(Particle::new(18, 25, crate::particle::ParticleKind::Sand(false)));
           self.world.set_particle(Particle::new(25, 50, crate::particle::ParticleKind::Sand(true)));
           self.world.set_particle(Particle::new(18, 20, crate::particle::ParticleKind::Sand(false)));
        }
    }

    pub fn draw (&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.world.draw(&mut self.canvas);
        self.canvas.present();
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

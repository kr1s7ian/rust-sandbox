extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::particles::gravel::Gravel;
use crate::particles::particle::{ParticleBehaviour, Particle};
use crate::particles::sand::Sand;
use crate::particles::water::Water;

use super::world::World;

pub struct Game
{
    running: bool,
    sdl_ctx: Sdl,
    canvas: WindowCanvas,
    world: World,
    timer: f32,
}

impl Game{
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

        //self.world.set_particle(Particle::<Air>::new(10,10));
        //self.world.set_particle(Particle::new(1, 1, Box::new(Sand{})));
    }

    pub fn poll_events(&mut self) {
        let mut events = self.sdl_ctx.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown{
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {self.world.tick()},
                _ => {},
            }
        }
    }

    pub fn update (&mut self) {
        //println!("updating");
        self.world.tick();
        self.timer += 0.1;

        if self.timer > 0.1 {
            self.timer = 0.0;
            self.world.set_particle(Particle::new(50, 1, Box::new(Sand{})));
            self.world.set_particle(Particle::new(20, 1, Box::new(Sand{})));
            self.world.set_particle(Particle::new(10, 1, Box::new(Gravel{})));
            self.world.set_particle(Particle::new(100, 1, Box::new(Water{})));
            self.world.set_particle(Particle::new(80, 1, Box::new(Sand{})));
        }
    }

    pub fn draw (&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.world.draw(&mut self.canvas);
        self.canvas.present();
        self.canvas.clear();
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

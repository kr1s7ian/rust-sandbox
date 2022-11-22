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

use crate::particle::{Particle, ParticleKind};
use crate::particles::sand::Sand;
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
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error constructing canvas : {:?},", e);
                std::process::exit(-1);
            }
        };

        let world = World::new(width / cell_size, height / cell_size, cell_size);

        Self {
            running: false,
            sdl_ctx,
            canvas,
            world,
            timer: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn poll_events(&mut self) {
        let mut events = self.sdl_ctx.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.world.translate(-1.0, 0.0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.world.translate(1.0, 0.0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    self.world.translate(0.0, 1.0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.world.translate(0.0, -1.0);
                }
                _ => {}
            }
        }

        let state = events.mouse_state();
        let mut solid = false;
        if state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
            solid = true;
        } else if state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Right) {
            solid = false;
        }

        if state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left)
            || state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Right)
        {
            let coord = self.world.window_to_world_coordinate(state.x(), state.y());
            if coord.is_some() {
                let coord = coord.unwrap();
                for x in 0..2 {
                    for y in 0..2 {
                        println!("{}", self.timer);
                        if self.timer > 0.2 {
                            self.world.set_particle(Particle::new::<Sand>(
                                coord.x as u32 + x,
                                coord.y as u32 + y,
                                solid
                            ));
                        }
                    }
                }
                println!("mouseX: {}, mouseY{}", state.x(), state.y());
            }
        }
    }

    pub fn update(&mut self) {
        //println!("updating");
        self.world.tick();
        self.timer += 0.1;

        if self.timer > 0.3 {
            self.timer = 0.0;
            self.world.set_particle(Particle::new::<Sand>(10,10, false));
        }
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.world.draw(&mut self.canvas);
        self.canvas.present();
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

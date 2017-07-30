extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

use gol_world::World;
use gol_controller::GoLController;

mod gol_world;
mod gol_controller;


fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Game of Life", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let gol_world = World::new();
    let mut gol_controller = GoLController::new(gol_world);

    let mut events = Events::new(EventSettings::new().lazy(false));
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        gol_controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
            });
        }
    }
}

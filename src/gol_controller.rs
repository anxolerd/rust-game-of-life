//! Game of Life controller
use piston::input::GenericEvent;

use World;

/// Handles events for Game of Life
pub struct GoLController {
    /// Stores the current world state
    pub world: World,

    /// Stores whether GoL is playing
    pub is_playing: bool,

    /// Stores last mouse cursor position
    cursor_pos: [f64; 2],

    time: f64,
    interval: f64,
}


impl GoLController {
    pub fn new(world: World) -> GoLController {
        return GoLController {
            world: world,
            is_playing: false,
            cursor_pos: [0.0; 2],

            time: 0.0,
            interval: 0.2,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key};
        if self.is_playing {
            if let Some(args) = e.update_args() {
                self.time = self.time + args.dt;
                if self.interval <= self.time {
                    println!("Timer event");
                    self.time = 0.0;
                    self.world.next_generation();
                }
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space { self.is_playing = !self.is_playing; }
            println!("Is Playing? {}", self.is_playing);
        }
    }
}

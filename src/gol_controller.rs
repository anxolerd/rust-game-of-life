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

    pub fn event<E: GenericEvent>(&mut self, pos:[f64;2], size: f64, e: &E) {
        use piston::input::{Button, MouseButton, Key};
        if self.is_playing {
            if let Some(args) = e.update_args() {
                self.time = self.time + args.dt;
                if self.interval <= self.time {
                    self.time = 0.0;
                    self.world.next_generation();
                }
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if !self.is_playing {
           if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
                let x = self.cursor_pos[0] - pos[0];
                let y = self.cursor_pos[1] - pos[1];

                // Are we inside board?
                if x >= 0.0 && x < size && y >= 0.0 && y < size {
                    let cell_x = (x / size * self.world.size as f64) as usize;
                    let cell_y = (y / size * self.world.size as f64) as usize;
                    let ind = [cell_x, cell_y];
                    let cell_value = self.world.get(ind);
                    self.world.set(ind, !cell_value);
                }
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space { self.is_playing = !self.is_playing; }
        }
    }
}

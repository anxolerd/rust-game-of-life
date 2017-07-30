//! GoL World View.

use graphics::types::Color;
use graphics::{Context, Graphics};

use GoLController;


pub struct GoLViewSettings {
    pub position: [f64; 2],
    pub size: f64,

    pub bg_color: Color,
    pub border_color: Color,
    pub border_radius: f64,

    pub alive_cell_color: Color,
    pub dead_cell_color: Color,
}


impl GoLViewSettings {
    pub fn new() -> GoLViewSettings {
        return GoLViewSettings {
            position: [6.0; 2],
            size: 500.0,
            bg_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.2, 0.0, 0.0, 1.0],
            border_radius: 1.0,
            alive_cell_color: [0.0, 0.0, 0.0, 1.0],
            dead_cell_color: [1.0, 1.0, 1.0, 1.0],
        };
    }
}


pub struct GoLView {
    pub settings: GoLViewSettings,
}


impl GoLView {
    pub fn new(settings: GoLViewSettings) -> GoLView {
        return GoLView { settings: settings };
    }

    pub fn draw<G: Graphics>(&self, controller: &GoLController, 
                             c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};

        let ref settings = self.settings;
        let world_rect = [settings.position[0], settings.position[1],
                          settings.size, settings.size];
        // Background
        Rectangle::new(settings.bg_color)
            .draw(world_rect, &c.draw_state, c.transform, g);

        // Cells
        let cell_size = settings.size / controller.world.size as f64;

        let alive_cell_rect = Rectangle::new(settings.alive_cell_color);
        let dead_cell_rect = Rectangle::new(settings.dead_cell_color);
        for i in 0..controller.world.size {
            for j in 0..controller.world.size {
                let cell_rect = [settings.position[0] + i as f64 * cell_size,
                                 settings.position[1] + j as f64 * cell_size,
                                 cell_size, cell_size];
                match controller.world.get([i, j]) {
                    true => {
                        alive_cell_rect.draw(cell_rect, &c.draw_state,
                                             c.transform, g);
                    },
                    false => { 
                        dead_cell_rect.draw(cell_rect, &c.draw_state,
                                            c.transform, g);
                    },
                }
            }
        }
        
        // Cells borders
        let cell_edge = Line::new(settings.border_color, 
                                  settings.border_radius);
        for i in 1..controller.world.size {
            let x = settings.position[0] + i as f64 * cell_size;
            let y = settings.position[1] + i as f64 * cell_size;

            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);
            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Border
        Rectangle::new_border(settings.border_color, settings.border_radius)
            .draw(world_rect, &c.draw_state, c.transform, g);
    }
}

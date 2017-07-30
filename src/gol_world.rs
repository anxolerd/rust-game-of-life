//! Game of life world logic


/// Size of the GoL world
const SIZE: usize = 64;

/// Stores GoL world information
pub struct World {
    /// Stores the state of the cells.
    /// `false` means that cell is dead
    /// `true` means that cell is alive
    pub cells: [[bool; SIZE]; SIZE as usize],
    pub size: usize,

    /// buffer to calculate next state
    buf: [[bool; SIZE]; SIZE as usize],
}


impl World {
    /// Creates a new world
    pub fn new() -> World {
        return World {
            cells: [[false; SIZE]; SIZE],
            size: SIZE,
            buf: [[false; SIZE]; SIZE],
        }
    }

    /// Set value to cell
    pub fn set(&mut self, ind: [usize; 2], val: bool) {
        self.cells[ind[0]][ind[1]] = val;
    }

    /// Get value from cell
    pub fn get(&self, ind: [usize; 2]) -> bool {
        return self.cells[ind[0]][ind[1]];
    }

    pub fn next_generation(&mut self) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                let alive_neighbours = self.count_alive_neighbours(x, y);
                let is_alive = match (self.get([x, y]), alive_neighbours) {
                    (true, 2) => true,
                    (_,    3) => true,
                    _ => false,
                };
                self.buf[x][y] = is_alive;
            }
        }
        for x in 0..SIZE {
            for y in 0..SIZE {
                self.cells[x][y] = self.buf[x][y];
            }
        }
    }

    fn count_alive_neighbours(&self, x:usize, y:usize) -> usize {
        return [
            [(SIZE + x - 1) % SIZE, (SIZE + y - 1) % SIZE],
            [(SIZE + x - 1) % SIZE, y              % SIZE],
            [(SIZE + x - 1) % SIZE, (y + 1)        % SIZE],
            [x,                     (SIZE + y - 1) % SIZE],
            [x,                     (y + 1)        % SIZE],
            [(x + 1)        % SIZE, (SIZE + y - 1) % SIZE],
            [(x + 1)        % SIZE, y              % SIZE],
            [(x + 1)        % SIZE, (y + 1)        % SIZE],
        ].iter()
         .filter(|&ind| ind[0] < SIZE && ind[1] < SIZE)
         .filter(|&ind| self.get(*ind))
         .count();
    }
}

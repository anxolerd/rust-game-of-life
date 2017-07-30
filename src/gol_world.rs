//! Game of life world logic
use std::collections::HashSet;

/// Size of the GoL world
const SIZE: usize = 64;

/// Stores GoL world information
pub struct World {
    /// Stores the state of the cells.
    /// `false` means that cell is dead
    /// `true` means that cell is alive
    pub cells: [[bool; SIZE]; SIZE as usize],
    pub size: usize,

    /// cells to check when calculating next generation
    cells_to_check: HashSet<[usize; 2]>,
}


impl World {
    /// Creates a new world
    pub fn new() -> World {
        return World {
            cells: [[false; SIZE]; SIZE],
            size: SIZE,
            cells_to_check: HashSet::new(),
        }
    }

    /// Set value to cell
    pub fn set(&mut self, ind: [usize; 2], val: bool) {
        self.cells[ind[0]][ind[1]] = val;
        self.cells_to_check.insert(ind);
        for &coords in self.neighbours(ind[0], ind[1]).iter() {
            self.cells_to_check.insert(coords);
        }
    }

    /// Get value from cell
    pub fn get(&self, ind: [usize; 2]) -> bool {
        return self.cells[ind[0]][ind[1]];
    }

    pub fn next_generation(&mut self) {
        let mut new_values: Vec<([usize; 2], bool)> = Vec::new();
        let mut cells_to_check = self.cells_to_check.clone();
        self.cells_to_check.clear();
        for coords in cells_to_check.drain() {
            let neighbours = self.neighbours(coords[0], coords[1]);
            let alive_count = neighbours.iter()
                .filter(|&ind| self.get(*ind))
                .count();
            let old_value = self.get(coords);
            let new_value = match (old_value, alive_count) {
                (true, 2) => true,
                (_,    3) => true,
                _ => false,
            };
            if new_value != old_value {
                new_values.push((coords, new_value));
            }
        }
        for &(coords, val) in new_values.iter() {
            self.set([coords[0], coords[1]], val);
        }
    }

    fn neighbours(&self, x:usize, y:usize) -> Vec<[usize; 2]> {
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
         .map(|&ind| ind)
         .collect();
    }
}

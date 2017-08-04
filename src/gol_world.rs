//! Game of life world logic
use std::collections::HashSet;

/// Size of the Game of Life world
const SIZE: usize = 64;

type Coords = [usize; 2];

/// Stores Game of Life world information
pub struct World {
    /// Stores the state of the cells.
    /// `false` means that cell is dead
    /// `true` means that cell is alive
    pub cells: [[bool; SIZE]; SIZE as usize],
    pub size: usize,

    /// cells to check when calculating next generation
    cells_to_check: HashSet<Coords>,
}


impl World {
    /// Creates a new world
    pub fn new() -> World {
        World {
            cells: [[false; SIZE]; SIZE],
            size: SIZE,
            cells_to_check: HashSet::new(),
        }
    }

    /// Set value to cell
    pub fn set(&mut self, ind: Coords, val: bool) {
        self.cells[ind[0]][ind[1]] = val;
        self.cells_to_check.insert(ind);
        for &coords in &self.neighbours(ind) {
            self.cells_to_check.insert(coords);
        }
    }

    /// Get value from cell
    pub fn get(&self, ind: Coords) -> bool {
        self.cells[ind[0]][ind[1]]
    }

    pub fn next_generation(&mut self) {
        let mut new_values: Vec<(Coords, bool)> = Vec::new();
        let mut cells_to_check = self.cells_to_check.clone();
        self.cells_to_check.clear();
        for coords in cells_to_check.drain() {
            let neighbours = self.neighbours(coords);
            let alive_count = neighbours.iter()
                .filter(|&ind| self.get(*ind))
                .count();
            let old_value = self.get(coords);
            let new_value = match (old_value, alive_count) {
                (true, 2) |
                (_,    3) => true,
                _ => false,
            };
            if new_value != old_value {
                new_values.push((coords, new_value));
            }
        }
        for &(coords, val) in &new_values {
            self.set(coords, val);
        }
    }

    fn neighbours(&self, ind: Coords) -> Vec<Coords> {
        let xs = [(SIZE + ind[0] - 1) % SIZE, ind[0], (ind[0] + 1) % SIZE];
        let ys = [(SIZE + ind[1] - 1) % SIZE, ind[1], (ind[1] + 1) % SIZE];
        vec![
            [xs[0], ys[0]], [xs[1], ys[0]], [xs[2], ys[0]],
            [xs[0], ys[1]],                 [xs[2], ys[1]],
            [xs[0], ys[2]], [xs[1], ys[2]], [xs[2], ys[2]],
        ]
    }
}

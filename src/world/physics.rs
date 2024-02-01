mod utils;
use utils::is_in_bounds_array;
use rand::Rng;

use super::world::{World, material::Material, material::MaterialTypes};

/// Physics engine for simulating the world.
pub struct PhysicsEngine {
    // Properties and settings for the physics engine
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self {  }
    }
}

impl PhysicsEngine {
    /// Creates a new PhysicsEngine instance.
    pub fn new() -> Self {
        Self {
            // Initialization of the physics engine
        }
    }

    /// Updates the world state based on physics rules.
    pub fn update_world(&mut self, world: &mut World) -> bool {
        let mut something_changed = false;
        // Implement physics logic here, like gravity, material interactions, etc.
        something_changed = self.apply_gravity_to_sand(world);
        return something_changed;
    }

    /// Applies gravity to sand particles in the world.
    pub fn apply_gravity_to_sand(&mut self, world: &mut World) -> bool {
        let mut something_changed = false;
        for x in (0..world.width).rev() {
            for y in (0..world.height - 1).rev() { // No need to check the bottom row
                if world.data[x][y].is_none() || world.data[x][y].unwrap().is_static {
                    continue;
                }

                // Check directly below, down-left, and down-right cells
                let below = y + 1;
                let down_left = if x > 0 { Some((x - 1, below)) } else { None };
                let down_right = if x < world.width - 1 { Some((x + 1, below)) } else { None };

                // Check the cell directly below
                if world.data[x][below].is_none() {
                    // Move sand particle down
                    something_changed = world.move_material(x, y, x, below);
                } else if world.data[x][below].unwrap().material_type == MaterialTypes::BlackHole {
                    world.data[x][y] = None;
                    something_changed = true;
                } else {
                    // Randomly choose to check left or right first for more natural behavior
                    let move_left_first = rand::thread_rng().gen_range(0..2) == 0;

                    if move_left_first && down_left.map_or(false, |(dx, dy)| world.data[dx][dy].is_none()) {
                        // Move sand particle down-left if possible
                        something_changed = world.move_material(x, y, x - 1, below);
                    } else if down_right.map_or(false, |(dx, dy)| world.data[dx][dy].is_none()) {
                        // Move sand particle down-right if possible
                        something_changed = world.move_material(x, y, x + 1, below);
                    } else if !move_left_first && down_left.map_or(false, |(dx, dy)| world.data[dx][dy].is_none()) {
                        // Check down-left again if down-right wasn't possible
                        something_changed = world.move_material(x, y, x - 1, below);
                    }
                }
            }
        }
        return something_changed;
    }
}


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
    pub fn update_world(&mut self, world: &mut World) {
        // Implement physics logic here, like gravity, material interactions, etc.
        self.apply_gravity_to_sand(world)
    }

    /// Applies gravity to sand particles in the world.
    pub fn apply_gravity_to_sand(&mut self, world: &mut World) {
        for x in (0..world.width).rev() {
            for y in (0..world.height).rev() {
                if world.data[x][y].is_none() {
                    continue;
                }

                let current = world.data[x][y].unwrap();

                if current.is_static {
                    continue;
                }

                // Material::sand(Rect::from_min_size(
                //     Pos2 { x: current.rect.left_top().x - self.cell_size, y: current.rect.left_top().y + self.cell_size },
                //     egui::Vec2::splat(self.cell_size),
                // ));

                // Check the cell directly below
                if y < world.height - 1 && world.data[x][y + 1].is_none() {
                    // Move sand particle down
                    world.data[x][y + 1] = Some(current);
                    world.data[x][y] = None;
                } else if y < world.height - 1 && x > 0 && world.data[x - 1][y + 1].is_none() {
                    // Move sand particle down-left if possible
                    world.data[x - 1][y + 1] = Some(current);
                    world.data[x][y] = None;
                } else if y < world.height - 1 && x < world.width - 1 && world.data[x + 1][y + 1].is_none() {
                    // Move sand particle down-right if possible
                    world.data[x + 1][y + 1] = Some(current);
                    world.data[x][y] = None;
                }
            }
        }
    }
    // Additional physics-related functionalities can be added here
}

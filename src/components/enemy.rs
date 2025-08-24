use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: u32,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy { health: 100 }
    }
}

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player { speed: 5.0 }
    }
}

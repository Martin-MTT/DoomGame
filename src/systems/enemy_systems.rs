use crate::components::Enemy;
use bevy::prelude::*;

pub fn print_enemy_health(query: Query<&Enemy>) {
    for enemy in query.iter() {
        println!("Enemy health: {}", enemy.health);
    }
}

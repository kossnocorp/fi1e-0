mod memory;
mod sense;

use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Experience(pub i32);

#[derive(Bundle)]
pub struct Character {
    pub name: Name,
    pub experience: Experience,
}

impl Character {
    pub fn new(name: String, experience: i32) -> Self {
        Self {
            name: Name(name),
            experience: Experience(experience),
        }
    }
}

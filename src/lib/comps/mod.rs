use bevy::ecs::component::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health{
    pub hp: i32,
}


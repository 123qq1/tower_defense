use bevy_prototype_lyon::prelude::*;
use bevy::prelude::*;
use crate::lib::comps;

pub fn draw_player_base(
    mut commands: Commands,
){
    let center = Vec2::new(0.0,0.0);
    let radii = Vec2::new(350.0,350.0);

    let player_shape = shapes::Ellipse{
        radii,
        center
    };

    let player_transform = Transform::from_xyz(0.0,0.0,10.0);

    commands.spawn()
        .insert(comps::Player)
        .insert(comps::Health {
            hp: 100,
        })
        .insert_bundle(GeometryBuilder::build_as(
        &player_shape,
        DrawMode::Fill(FillMode::color(Color::CYAN)),
        player_transform,
    ));
}

pub fn setup(
    mut commands: Commands,
){
    commands.spawn_bundle(Camera2dBundle::default());
}
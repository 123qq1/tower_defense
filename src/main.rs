mod lib;

use bevy::prelude::*;

fn main() {
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugins(lib::LibGroup)

        .insert_resource(WindowDescriptor{
            title: "3d Test".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })

        .run();
}

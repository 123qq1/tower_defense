mod startup;
mod comps;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct LibGroup;
struct StartUpPlugin;


impl PluginGroup for LibGroup{
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(StartUpPlugin);
    }
}

impl Plugin for StartUpPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(startup::draw_player_base)
            .add_startup_system(startup::setup)
        ;
    }
}
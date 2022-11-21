use bevy::prelude::*;

pub mod entities;
pub mod systems;

use crate::systems::init::setup;
use crate::systems::gloop::get_game_objects;

fn main() {
  App::new()
      .add_plugins(DefaultPlugins)
      .add_startup_system(setup)
      .add_system(get_game_objects)
      .run();
}

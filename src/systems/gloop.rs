use bevy::prelude::*;
use crate::entities::field::Position;

pub fn get_game_objects(
  mut query: Query<(&Position,&mut Transform)>,
)
{
  for (_position, mut transform) in query.iter_mut() {
    transform.rotate_x(0.009);
  }
}

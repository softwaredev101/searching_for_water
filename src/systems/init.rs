use bevy::prelude::*;
use crate::entities::field::Position;

fn init_field(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
  let x: i32 = 3;
  let y: i32 = 3;
  let mut x_off_set = -1.0;
  let mut y_off_set = 0.25;
  
  for _row in 0..x {
    for _col in 0..y {
      commands.spawn(SceneBundle {
        scene: asset_server.load("3d/Cube.glb#Scene0"),
        transform: Transform::from_xyz(x_off_set,y_off_set,0.0),
        ..default()
      }).insert(Position{x:x,y:y});
      x_off_set += 3.0;
    }
    x_off_set = -1.0;
    y_off_set += 3.0;
  }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn( PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            range: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0 , 3.25, 3.0),
        ..default()
    });

    commands.spawn( Camera3dBundle {
        transform: Transform::from_xyz(2.0 , 3.25, 15.0).looking_at(Vec3{x:2.0 ,y:3.25,z:0.0}, Vec3::Y),
        ..default()
    });

   init_field(commands, asset_server);
}

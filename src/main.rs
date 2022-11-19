use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_field)
        .add_startup_system(setup)
        .run();
}

fn init_field(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
  let mut x: i32 = 3;
  let mut y: i32 = 3;
  let mut x_off_set = -1.0;
  let mut y_off_set = 0.25;
  
  while x > 0 {
    while y > 0 {
      commands.spawn(SceneBundle {
        scene: asset_server.load("3d/Cube.glb#Scene0"),
        transform: Transform::from_xyz(x_off_set,y_off_set,0.0),
        ..default()
      });
      x_off_set += 3.0;
      y -= 1;
    }
    x_off_set = -1.0;
    y_off_set += 3.0;
    y = 3;
    x = x-1;
  }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn( PointLightBundle {
        point_light: PointLight {
            intensity: 750.0,
            range: 150.0,
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
}

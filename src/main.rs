use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

use rand::prelude::random;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    for _i in 0..10 {
        // sphere
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: random::<f32>(), ..Default::default() })),
            material: materials.add(Color::rgb(random::<f32>(), random::<f32>(), random::<f32>()).into()),
            transform: 
                Transform::from_xyz(
                    random::<f32>() * 10., 
                    random::<f32>() * 10., 
                    random::<f32>() * 10.
                ),
            ..Default::default()
        });
    }

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());
}

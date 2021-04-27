use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

use rand_distr::{Distribution, UnitSphere};

const NUM_STARS: i32 = 100;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2.,
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(setup_stars.system())
        .run();
}

fn setup_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let star_pos: Vec<Vec3> = UnitSphere
        .sample_iter(&mut rng)
        .take(NUM_STARS as usize)
        .map(|xyz| 800. * Vec3::new(xyz[0], xyz[1], xyz[2]))
        .collect();

    star_pos.into_iter().for_each(|pos| {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 1,
                ..Default::default()
            })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(pos),
            ..Default::default()
        });
    });
}

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // sun
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 10.0,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(Light {
            intensity: 50_000.,
            range: 2000.,
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

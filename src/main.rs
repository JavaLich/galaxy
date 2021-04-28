use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::AmbientLight;
use bevy::{prelude::*, render::mesh::shape};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

use rand_distr::{Distribution, UnitSphere};

const NUM_STARS: i32 = 0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            vsync: true,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2.,
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_solar_system.system())
        .add_startup_system(setup_stars.system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());
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
                subdivisions: 2,
                ..Default::default()
            })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(pos),
            ..Default::default()
        });
    });

    let skybox = meshes.add(Mesh::from(shape::Plane { size: 100.0 }));
    let mut transform = Transform::from_xyz(0.0, 0.0, -50.0);

    transform.rotate(Quat::from_axis_angle(Vec3::X, 3.14 / 2.));
    commands.spawn_bundle(PbrBundle {
        mesh: skybox.clone(),
        material: materials.add(Color::DARK_GRAY.into()),
        transform: transform.clone(),
        ..Default::default()
    });

    transform = Transform::from_xyz(0.0, 0.0, 50.0);
    transform.rotate(Quat::from_axis_angle(Vec3::X, -3.14 / 2.));
    commands.spawn_bundle(PbrBundle {
        mesh: skybox.clone(),
        material: materials.add(Color::DARK_GRAY.into()),
        transform: transform.clone(),
        ..Default::default()
    });
}

fn setup_solar_system(
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
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                roughness: 0.6,
                emissive: Color::YELLOW,
                ..Default::default()
            }),
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(Light {
            intensity: 50_000.,
            range: 2000.,
            ..Default::default()
        });

    // planet
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.5,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            roughness: 0.6,
            emissive: Color::GREEN,
            ..Default::default()
        }),
        transform: Transform::from_xyz(20.0, 8.0, 4.0),
        ..Default::default()
    });
}

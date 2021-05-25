use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::AmbientLight;
use bevy::{prelude::*, render::mesh::shape};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

mod background;
mod generation;

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
        .add_asset::<background::BackgroundMaterial>()
        .add_asset::<generation::planet::PlanetMaterial>()
        .add_startup_system(background::setup_background.system())
        .add_startup_system(setup_camera.system())
        .add_startup_system(setup_solar_system.system())
        .add_startup_system(generation::planet::setup_planets.system())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(10.0, 10.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());
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
            range: 5000.,
            ..Default::default()
        });
}

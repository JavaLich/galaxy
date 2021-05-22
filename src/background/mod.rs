use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};

use rand_distr::{Distribution, UnitSphere};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0dbbc306-413c-4a2f-8f24-4a8212205619"]
pub struct BackgroundMaterial;

pub fn setup_background(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<BackgroundMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    asset_server.watch_for_changes().unwrap();

    let mut rng = rand::thread_rng();

    let stars: Vec<Vec3> = UnitSphere
        .sample_iter(&mut rng)
        .take(1000)
        .map(|xyz| 800.0 * Vec3::new(xyz[0], xyz[1], xyz[2]))
        .collect();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/background.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/background.frag")),
    }));

    let material = custom_materials.add(BackgroundMaterial);

    render_graph.add_system_node(
        "background_material",
        AssetRenderResourcesNode::<BackgroundMaterial>::new(true),
    );

    render_graph
        .add_node_edge("background_material", base::node::MAIN_PASS)
        .unwrap();

    stars.into_iter().for_each(|s| {
        commands
            .spawn_bundle(MeshBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.,
                    subdivisions: 2,
                })),
                transform: Transform::from_translation(s),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline_handle.clone(),
                )]),
                ..Default::default()
            })
            .insert(material.clone());
    });
}

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

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "aa030eb0-c8cb-4ce3-9faa-0f74d4be2280"]
pub struct PlanetMaterial;

pub fn setup_planets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<PlanetMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    asset_server.watch_for_changes().unwrap();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/planet.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/planet.frag")),
    }));

    let material = custom_materials.add(PlanetMaterial);

    render_graph.add_system_node(
        "planet_material",
        AssetRenderResourcesNode::<PlanetMaterial>::new(true),
    );

    render_graph
        .add_node_edge("planet_material", base::node::MAIN_PASS)
        .unwrap();

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.5,
                ..Default::default()
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle.clone(),
            )]),
            transform: Transform::from_xyz(-20.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(material);
}

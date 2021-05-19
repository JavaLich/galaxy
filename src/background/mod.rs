use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices,
        pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};

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

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/background.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/background.frag")),
    }));

    let size = 1.;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, size, 0.0],
            [0.0, 0.0, 0.0],
            [size, size, 0.0],
            [size, 0.0, 0.0],
            [0.0, 0.0, size],
            [size, 0.0, size],
            [0.0, size, size],
            [size, size, size],
            [0.0, size, 0.0],
            [size, size, 0.0],
            [0.0, size, 0.0],
            [0.0, size, size],
            [size, size, 0.0],
            [size, size, size],
        ],
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 0.66],
            [0.25, 0.66],
            [0.0, 0.33],
            [0.25, 0.33],
            [0.5, 0.66],
            [0.5, 0.33],
            [0.75, 0.66],
            [0.75, 0.33],
            [1.0, 0.66],
            [1.0, 0.33],
            [0.25, 1.0],
            [0.5, 1.0],
            [0.25, 0.0],
            [0.5, 0.0],
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![
        0, 2, 1, 1, 2, 3, 4, 5, 6, 5, 7, 6, 6, 7, 8, 7, 9, 8, 1, 3, 4, 3, 5, 4, 1, 11, 10, 1, 4,
        11, 3, 12, 5, 5, 12, 13,
    ])));

    let material = custom_materials.add(BackgroundMaterial);

    let size = 500.;
    let mut transform = Transform::from_xyz(size / 2., size / 2., size / 2.);
    transform.scale = Vec3::new(-size, -size, -size);

    render_graph.add_system_node(
        "background_material",
        AssetRenderResourcesNode::<BackgroundMaterial>::new(true),
    );

    render_graph
        .add_node_edge("background_material", base::node::MAIN_PASS)
        .unwrap();

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform,
            ..Default::default()
        })
        .insert(material);
}

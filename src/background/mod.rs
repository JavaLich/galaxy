use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0dbbc306-413c-4a2f-8f24-4a8212205619"]
pub struct BackgroundMaterial {
    pub color: Color,
}

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

    render_graph.add_system_node(
        "background_material",
        AssetRenderResourcesNode::<BackgroundMaterial>::new(true),
    );

    render_graph
        .add_node_edge("background_material", base::node::MAIN_PASS)
        .unwrap();

    // cube
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1000.0 }));
    let material = custom_materials.add(BackgroundMaterial {
        color: Color::DARK_GRAY,
    });

    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    transform.scale = Vec3::new(-1., -1., -1.);

    commands
        .spawn_bundle(MeshBundle {
            mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform,
            ..Default::default()
        })
        .insert(material);
}

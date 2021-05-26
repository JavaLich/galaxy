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
#[uuid = "aa030eb0-c8cb-4ce3-9faa-0f74d4be2280"]
pub struct PlanetMaterial;

fn generate_mesh() -> Mesh {
    let size = 1.;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let verts: Vec<[f32; 3]> = (0..8)
        .map(|i| {
            [
                if i & 4 != 0 { size } else { -size },
                if i & 2 != 0 { size } else { -size },
                if i & 1 != 0 { size } else { -size },
            ]
        })
        .collect();

    let mut tris: Vec<[f32; 3]> = Vec::new();
    for i in 0..3 {
        let v1 = 1 << i;
        let v2 = if v1 == 4 { 1 } else { v1 << 1 };

        tris.push(verts[0]);
        tris.push(verts[v1]);
        tris.push(verts[v2]);
        tris.push(verts[v1 + v2]);
        tris.push(verts[v2]);
        tris.push(verts[v1]);
        tris.push(verts[7]);
        tris.push(verts[7 - v2]);
        tris.push(verts[7 - v1]);
        tris.push(verts[7 - (v1 + v2)]);
        tris.push(verts[7 - v1]);
        tris.push(verts[7 - v2]);
    }

    let len = tris.len();

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, tris);
    mesh.set_indices(Some(Indices::U32((0..36).collect())));

    let uv: Vec<[f32; 2]> = (0..len).map(|_x| [0.; 2]).collect();
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);

    mesh
}

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
            mesh: meshes.add(generate_mesh()),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(-20.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(material);
}

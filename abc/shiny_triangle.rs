use bevy::{
    prelude::*, 
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create mesh
    let mut mesh = Mesh::from(Triangle2d::default());

    // Build vertex colors for the triangle
    let vertex_colors: Vec<[f32; 4]> = vec![
        Color::RED.as_rgba_f32(),
        Color::GREEN.as_rgba_f32(),
        Color::BLUE.as_rgba_f32(),
    ];

    // Insert the vertex colors as an attribute
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    // Create camera
    commands.spawn(Camera2dBundle::default());

    // Create triangle
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle.clone(),
        material: materials.add(ColorMaterial::default()),
        transform: Transform::default().with_scale(Vec3::splat(512.)),
        ..default()
    });
}
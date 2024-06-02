use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
enum Direction {
    Down,
}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Platform;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create camera
    commands.spawn(Camera2dBundle::default());

    // Create circle
    let circle = Mesh::from(Circle::default());

    let circle_handle: Mesh2dHandle = meshes.add(circle).into();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: circle_handle.clone(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.5, 0.5, 1.0))),
            transform: Transform::default().with_scale(Vec3::splat(32.0)),
            ..default()
        },
        Direction::Down,
        Velocity(Vec3::new(0.0, -100.0, 0.0)),
    ));

    // Create platform
    let platform = Mesh::from(Rectangle::new(400.0, 5.0));

    let platform_handle: Mesh2dHandle = meshes.add(platform).into();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: platform_handle.clone(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.8, 0.3, 0.3))),
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
            ..default()
        },
        Platform,
    ));
}

fn update(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), Without<Platform>>, platform_query: Query<&Transform, With<Platform>>) {
    let platform_transform = platform_query.single();
    let platform_top = platform_transform.translation.y + 2.5; // Half of the platform height

    for (mut transform, velocity) in &mut query {
        // Update position
        transform.translation += velocity.0 * time.delta_seconds();

        // Simple collision detection with platform
        if transform.translation.y - 16.0 <= platform_top { // Assuming circle radius is 16.0
            transform.translation.y = platform_top + 16.0;
        }
    }
}

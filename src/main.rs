use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
struct Snake;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

fn main() {
    println!("Starting Bevy Snake!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_snake)
        .run();
}

fn setup_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let box_mesh = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    let box_color = Color::rgb(0.4, 0.2, 0.1);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: box_mesh,
            material: materials.add(box_color),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Snake,
        Velocity(INITIAL_BALL_DIRECTION.normalize()),
    ));
}

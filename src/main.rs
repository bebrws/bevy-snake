use bevy::{
    app,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeBody;

#[derive(Component, Clone, Copy, PartialEq)]
struct Position(Vec2);

#[derive(Component)]
struct Apple;

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 1000.0;
const OBJECT_SIZE: f32 = 10.0;

fn main() {
    println!("Starting Bevy Snake!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [WINDOW_WIDTH, WINDOW_HEIGHT].into(),
                title: "Bevy Snake!".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup_snake)
        .run();
}
fn get_random_position() -> Vec2 {
    let mut rng = rand::thread_rng();
    let x = (rng.gen_range(0..(WINDOW_WIDTH / OBJECT_SIZE) as i32)
        - (((WINDOW_WIDTH / OBJECT_SIZE) as i32) / 2)) as f32
        * OBJECT_SIZE;
    let y = (rng.gen_range(0..(WINDOW_HEIGHT / OBJECT_SIZE) as i32)
        - (((WINDOW_HEIGHT / OBJECT_SIZE) as i32) / 2)) as f32
        * OBJECT_SIZE;
    Vec2::new(x, y)
}

fn setup_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let head_mesh = Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0)));
    let box_color = Color::rgb(0.8, 0.2, 0.1);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: head_mesh,
            material: materials.add(box_color),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        SnakeHead {
            direction: Direction::Up,
        },
        Position(Vec2::new(0.0, 0.0)),
    ));

    let body_mesh = Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0)));
    let box_color = Color::rgb(0.8, 0.2, 0.1);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: body_mesh,
            material: materials.add(box_color),
            transform: Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)),
            ..default()
        },
        SnakeBody,
        Position(Vec2::new(0.0, -10.0)),
    ));

    let mut apple_position = get_random_position();
    loop {
        if apple_position != Vec2::new(0.0, 0.0) && apple_position != Vec2::new(0.0, -10.0) {
            break;
        }
        apple_position = get_random_position();
    }
    let apple_mesh = Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0)));
    let box_color = Color::rgb(0.0, 0.8, 0.0);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: apple_mesh,
            material: materials.add(box_color),
            transform: Transform::from_translation(apple_position.extend(0.0)),
            ..default()
        },
        SnakeBody,
        Position(apple_position),
    ));
}

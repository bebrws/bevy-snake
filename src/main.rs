use bevy::{
    app,
    asset::transformer,
    ecs::query,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;
use std::time::Duration;

#[derive(Clone, PartialEq)]
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

#[derive(Component)]
struct Apple;

const OBJECT_SIZE: f32 = 20.0;
const WINDOW_WIDTH: f32 = 50.0 * OBJECT_SIZE;
const WINDOW_HEIGHT: f32 = 50.0 * OBJECT_SIZE;
const SNAKE_SPEED: f32 = 200.0;

fn main() {
    println!("Starting Bevy Snake!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#bevy-portal".to_string()),
                resolution: [WINDOW_WIDTH + OBJECT_SIZE, WINDOW_HEIGHT + OBJECT_SIZE].into(),
                title: "Bevy Snake!".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup_snake)
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(100)))
        .add_systems(FixedUpdate, (handle_input, move_snake, check_collisions))
        .run();
}

fn check_collisions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut apple_query: Query<(&mut Transform), (With<Apple>, Without<SnakeHead>, Without<SnakeBody>)>,
    mut snake_head_query: Query<(&mut Transform, &mut SnakeHead)>,
    mut snake_body_query: Query<(&mut Transform, Entity), (With<SnakeBody>, Without<SnakeHead>)>,
) {
    let apple_translation = apple_query.single_mut().translation;
    let snake_head_translation = snake_head_query.single_mut().0.translation;
    let (mut snake_head_transform, mut snake_head) = snake_head_query.single_mut();

    let mut crashed = false;
    for (body_transform, body) in &snake_body_query {
        if snake_head_translation == body_transform.translation {
            crashed = true;
        }
    }

    if crashed {
        snake_body_query.iter().for_each(|(body_transform, body)| {
            commands.entity(body).despawn();
        });
    }

    if snake_head_translation.x < -WINDOW_WIDTH / 2.0 {
        snake_head.direction = Direction::Left;
        snake_head_transform.translation =
            Vec3::new(WINDOW_WIDTH / 2.0, snake_head_translation.y, 0.0);
    }
    if snake_head_translation.x > WINDOW_WIDTH / 2.0 {
        snake_head.direction = Direction::Right;
        snake_head_transform.translation =
            Vec3::new(-WINDOW_WIDTH / 2.0, snake_head_translation.y, 0.0);
    }
    if snake_head_translation.y < -WINDOW_HEIGHT / 2.0 {
        snake_head.direction = Direction::Down;
        snake_head_transform.translation =
            Vec3::new(snake_head_translation.x, WINDOW_WIDTH / 2.0, 0.0);
    }
    if snake_head_translation.y > WINDOW_HEIGHT / 2.0 {
        snake_head.direction = Direction::Up;
        snake_head_transform.translation =
            Vec3::new(snake_head_translation.x, -WINDOW_WIDTH / 2.0, 0.0);
    }

    if apple_translation == snake_head_translation {
        let mut apple_position = get_random_position();
        loop {
            if apple_position != Vec2::new(0.0, 0.0)
                && apple_position != Vec2::new(0.0, -OBJECT_SIZE)
            {
                break;
            }
            apple_position = get_random_position();
        }
        apple_query.single_mut().translation = apple_position.extend(0.0);

        let body_mesh = Mesh2dHandle(meshes.add(Rectangle::new(OBJECT_SIZE, OBJECT_SIZE)));
        let box_color = Color::rgb(0.8, 0.2, 0.1);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: body_mesh,
                material: materials.add(box_color),
                transform: Transform::from_translation(apple_position.extend(0.0)),
                ..default()
            },
            SnakeBody,
        ));
    }
}

fn move_snake(
    mut snake_head_query: Query<(&mut Transform, &mut SnakeHead)>,
    mut snake_body_query: Query<(&mut Transform), (With<SnakeBody>, Without<SnakeHead>)>,
    time: Res<Time>,
) {
    // println!("time.delta_seconds(): {}", time.delta_seconds());
    let mut moved_head = false;
    let mut last_translation = snake_head_query.single().0.translation.clone();

    // println!(
    //     "Moving {}",
    //     (((SNAKE_SPEED * time.delta_seconds() / OBJECT_SIZE) as i32) as f32) * OBJECT_SIZE
    // );
    let mut transform_and_snake_head = snake_head_query.single_mut();
    let mut snake_head_transform = transform_and_snake_head.0;
    let shake_head_translation = snake_head_transform.translation.clone();
    let snake_head = transform_and_snake_head.1;
    match snake_head.direction {
        Direction::Up => {
            snake_head_transform.translation.y +=
                (((SNAKE_SPEED * time.delta_seconds() / OBJECT_SIZE) as i32) as f32) * OBJECT_SIZE;
        }
        Direction::Down => {
            snake_head_transform.translation.y -=
                (((SNAKE_SPEED * time.delta_seconds() / OBJECT_SIZE) as i32) as f32) * OBJECT_SIZE;
        }
        Direction::Left => {
            snake_head_transform.translation.x -=
                (((SNAKE_SPEED * time.delta_seconds() / OBJECT_SIZE) as i32) as f32) * OBJECT_SIZE;
        }
        Direction::Right => {
            snake_head_transform.translation.x +=
                (((SNAKE_SPEED * time.delta_seconds() / OBJECT_SIZE) as i32) as f32) * OBJECT_SIZE;
        }
    }

    let mut last_translation = shake_head_translation;

    snake_body_query.iter_mut().for_each(|mut body_transform| {
        let temp_translation = body_transform.translation.clone();
        body_transform.translation = last_translation;
        last_translation = temp_translation;
    });
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_head_query: Query<(&mut Transform, &mut SnakeHead)>,
    mut snake_body_query: Query<(&mut Transform), (With<SnakeBody>, Without<SnakeHead>)>,
    time: Res<Time>,
) {
    let mut moved_head = false;
    let mut last_translation = snake_head_query.single().0.translation.clone();
    keyboard_input.get_pressed().for_each(|key| {
        snake_head_query
            .iter_mut()
            .for_each(|(mut transform, mut snake_head)| match key {
                KeyCode::ArrowUp => {
                    if snake_head.direction != Direction::Down {
                        moved_head = true;
                        snake_head.direction = Direction::Up;
                        // transform.translation.y += OBJECT_SIZE;
                    }
                }
                KeyCode::ArrowDown => {
                    if snake_head.direction != Direction::Up {
                        moved_head = true;
                        snake_head.direction = Direction::Down;
                        // transform.translation.y -= OBJECT_SIZE;
                    }
                }
                KeyCode::ArrowLeft => {
                    if snake_head.direction != Direction::Right {
                        moved_head = true;
                        snake_head.direction = Direction::Left;
                        // transform.translation.x -= OBJECT_SIZE;
                    }
                }
                KeyCode::ArrowRight => {
                    if snake_head.direction != Direction::Left {
                        moved_head = true;
                        snake_head.direction = Direction::Right;
                        // transform.translation.x += OBJECT_SIZE;
                    }
                }
                _ => {}
            })
    });
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

    let head_mesh = Mesh2dHandle(meshes.add(Rectangle::new(OBJECT_SIZE, OBJECT_SIZE)));
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
    ));

    let body_mesh = Mesh2dHandle(meshes.add(Rectangle::new(OBJECT_SIZE, OBJECT_SIZE)));
    let box_color = Color::rgb(0.8, 0.2, 0.1);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: body_mesh,
            material: materials.add(box_color),
            transform: Transform::from_translation(Vec3::new(0.0, -OBJECT_SIZE, 0.0)),
            ..default()
        },
        SnakeBody,
    ));

    let mut apple_position = get_random_position();
    loop {
        if apple_position != Vec2::new(0.0, 0.0) && apple_position != Vec2::new(0.0, -OBJECT_SIZE) {
            break;
        }
        apple_position = get_random_position();
    }
    let apple_mesh = Mesh2dHandle(meshes.add(Rectangle::new(OBJECT_SIZE, OBJECT_SIZE)));
    let box_color = Color::rgb(0.0, 0.8, 0.0);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: apple_mesh,
            material: materials.add(box_color),
            transform: Transform::from_translation(apple_position.extend(0.0)),
            ..default()
        },
        Apple,
    ));
}

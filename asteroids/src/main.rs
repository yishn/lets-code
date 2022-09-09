use std::f32::consts::PI;
use bevy::{
  input::{keyboard::KeyboardInput, ButtonState},
  prelude::*,
  render::{mesh::Indices, render_resource::PrimitiveTopology},
  sprite::MaterialMesh2dBundle,
};

const VIEWPORT_WIDTH: usize = 1280;
const VIEWPORT_HEIGHT: usize = 720;
const VIEWPORT_MAX_X: f32 = VIEWPORT_WIDTH as f32 / 2.0;
const VIEWPORT_MIN_X: f32 = -VIEWPORT_MAX_X;
const VIEWPORT_MAX_Y: f32 = VIEWPORT_HEIGHT as f32 / 2.0;
const VIEWPORT_MIN_Y: f32 = -VIEWPORT_MAX_Y;
const ASTEROID_VELOCITY: f32 = 2.0;
const BULLET_VELOCITY: f32 = 6.0;
const BULLET_DISTANCE: f32 = VIEWPORT_HEIGHT as f32 * 0.8;
const STARSHIP_ROTATION_SPEED: f32 = 5.0 * 2.0 * PI / 360.0;
const STARSHIP_ACCELERATION: f32 = 0.2;
const STARSHIP_DECELERATION: f32 = 0.01;
const STARSHIP_MAX_VELOCITY: f32 = 10.0;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(update_position)
    .add_system(remove_bullet)
    .add_system(decelerate_starship)
    .add_system(sync_translate_transform.after(update_position))
    .add_system(sync_asteroid_scale_transform)
    .add_system(sync_starship_rotation_transform)
    .add_system(keyboard_events)
    .add_system(detect_starship_asteroid_collision)
    .add_system(detect_bullet_asteroid_collision)
    .run();
}

#[derive(Debug, Clone, Copy)]
enum AsteroidSize {
  Big,
  Medium,
  Small,
}

impl AsteroidSize {
  fn scale(&self) -> f32 {
    match self {
      AsteroidSize::Big => 100.0,
      AsteroidSize::Medium => 65.0,
      AsteroidSize::Small => 30.0,
    }
  }
}

#[derive(Component)]
struct Starship {
  rotation_angle: f32,
}

impl Starship {
  fn direction(&self) -> Vec2 {
    let (y, x) = (self.rotation_angle + PI / 2.0).sin_cos();

    Vec2::new(x, y)
  }
}

#[derive(Component)]
struct Bullet {
  start: Vec2,
}

#[derive(Component)]
struct Asteroid {
  size: AsteroidSize,
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

fn create_starship_mesh() -> Mesh {
  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

  mesh.insert_attribute(
    Mesh::ATTRIBUTE_POSITION,
    vec![[0.0, 0.5, 0.0], [-0.25, -0.5, 0.0], [0.25, -0.5, 0.0]],
  );
  mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
  mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);
  mesh.insert_attribute(
    Mesh::ATTRIBUTE_UV_0,
    vec![[0.5, 0.0], [0.0, 1.0], [1.0, 1.0]],
  );

  mesh
}

fn get_random_point() -> Vec2 {
  Vec2::new(
    (rand::random::<f32>() * 2.0 - 1.0) * (VIEWPORT_WIDTH as f32) / 2.0,
    (rand::random::<f32>() * 2.0 - 1.0) * (VIEWPORT_HEIGHT as f32) / 2.0,
  )
}

fn setup(
  mut commands: Commands,
  mut windows: ResMut<Windows>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let window = windows.get_primary_mut().unwrap();
  window.set_resolution(VIEWPORT_WIDTH as f32, VIEWPORT_HEIGHT as f32);

  commands.spawn_bundle(Camera2dBundle::default());

  commands
    .spawn()
    .insert(Starship {
      rotation_angle: 0.0,
    })
    .insert(Position(Vec2::splat(0.0)))
    .insert(Velocity(Vec2::splat(0.0)))
    .insert_bundle(MaterialMesh2dBundle {
      mesh: meshes.add(create_starship_mesh()).into(),
      transform: Transform::default()
        .with_scale(Vec3::splat(50.0))
        .with_translation(Vec3::new(0.0, 0.0, 1.0)),
      material: materials
        .add(ColorMaterial::from(Color::rgba(1.0, 0.0, 0.0, 1.0))),
      ..default()
    });

  for _ in 0..6 {
    commands
      .spawn()
      .insert(Asteroid {
        size: AsteroidSize::Big,
      })
      .insert(Position(get_random_point()))
      .insert(Velocity(get_random_point().normalize() * ASTEROID_VELOCITY))
      .insert_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
        transform: Transform::default()
          .with_translation(Vec3::new(0.0, 0.0, 2.0)),
        material: materials
          .add(ColorMaterial::from(Color::rgba(0.8, 0.8, 0.8, 1.0))),
        ..default()
      });
  }
}

fn sync_translate_transform(mut query: Query<(&Position, &mut Transform)>) {
  for (position, mut transform) in &mut query {
    transform.translation =
      Vec3::new(position.0.x, position.0.y, transform.translation.z);
  }
}

fn sync_asteroid_scale_transform(
  mut query: Query<(&Asteroid, &mut Transform)>,
) {
  for (asteroid, mut transform) in &mut query {
    transform.scale = Vec3::splat(asteroid.size.scale())
  }
}

fn sync_starship_rotation_transform(
  mut query: Query<(&Starship, &mut Transform)>,
) {
  for (starship, mut transform) in &mut query {
    transform.rotation = Quat::from_rotation_z(starship.rotation_angle);
  }
}

fn update_position(mut query: Query<(&Velocity, &Transform, &mut Position)>) {
  for (velocity, transform, mut position) in &mut query {
    let mut new_position = position.0 + velocity.0;
    let half_scale = transform.scale.max_element() / 2.0;

    if new_position.x > VIEWPORT_MAX_X + half_scale {
      new_position.x = VIEWPORT_MIN_X - half_scale;
    } else if new_position.x < VIEWPORT_MIN_X - half_scale {
      new_position.x = VIEWPORT_MAX_X + half_scale;
    }

    if new_position.y > VIEWPORT_MAX_Y + half_scale {
      new_position.y = VIEWPORT_MIN_Y - half_scale;
    } else if new_position.y < VIEWPORT_MIN_Y - half_scale {
      new_position.y = VIEWPORT_MAX_Y + half_scale;
    }

    position.0 = new_position;
  }
}

fn keyboard_events(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  keys: Res<Input<KeyCode>>,
  mut key_evr: EventReader<KeyboardInput>,
  mut query: Query<(&mut Starship, &Position, &mut Velocity)>,
) {
  for (mut starship, starship_position, mut velocity) in &mut query {
    if keys.pressed(KeyCode::Left) {
      starship.rotation_angle += STARSHIP_ROTATION_SPEED;
    } else if keys.pressed(KeyCode::Right) {
      starship.rotation_angle -= STARSHIP_ROTATION_SPEED;
    }

    if keys.pressed(KeyCode::Up) {
      velocity.0 += starship.direction() * STARSHIP_ACCELERATION;

      if velocity.0.length() > STARSHIP_MAX_VELOCITY {
        velocity.0 = velocity.0.normalize_or_zero() * STARSHIP_MAX_VELOCITY;
      }
    }

    for evt in key_evr.iter() {
      if let (ButtonState::Pressed, Some(KeyCode::Space)) =
        (evt.state, evt.key_code)
      {
        commands
          .spawn()
          .insert(Bullet {
            start: starship_position.0.clone(),
          })
          .insert(Position(starship_position.0.clone()))
          .insert(Velocity(
            starship.direction().normalize() * BULLET_VELOCITY,
          ))
          .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
              .with_scale(Vec3::splat(5.0))
              .with_translation(Vec3::splat(0.0)),
            material: materials
              .add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 1.0))),
            ..default()
          });
      }
    }
  }
}

fn remove_bullet(
  mut commands: Commands,
  query: Query<(Entity, &Bullet, &Position)>,
) {
  for (entity, bullet, position) in &query {
    if (bullet.start - position.0).length() > BULLET_DISTANCE {
      commands.entity(entity).despawn();
    }
  }
}

fn decelerate_starship(
  keys: Res<Input<KeyCode>>,
  mut query: Query<&mut Velocity, With<Starship>>,
) {
  // Only decelerate when not accelerating
  if !keys.pressed(KeyCode::Up) {
    for mut velocity in &mut query {
      velocity.0 *= 1.0 - STARSHIP_DECELERATION;
    }
  }
}

fn detect_starship_asteroid_collision(
  mut commands: Commands,
  starship_query: Query<(Entity, &Transform, &Position), With<Starship>>,
  asteroids_query: Query<(&Transform, &Position), With<Asteroid>>,
) {
  for (starship_entity, starship_transform, starship_position) in
    &starship_query
  {
    for (asteroid_transform, asteroid_position) in &asteroids_query {
      let starship_size = starship_transform.scale.max_element();
      let asteroid_size = asteroid_transform.scale.max_element();
      let distance = (starship_position.0 - asteroid_position.0).length();

      if distance < starship_size / 4.0 + asteroid_size / 2.0 {
        commands.entity(starship_entity).despawn();
      }
    }
  }
}

fn detect_bullet_asteroid_collision(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  bullets_query: Query<(Entity, &Transform, &Position), With<Bullet>>,
  asteroids_query: Query<(Entity, &Asteroid, &Transform, &Position)>,
) {
  for (bullet_entity, bullet_transform, bullet_position) in &bullets_query {
    for (asteroid_entity, asteroid, asteroid_transform, asteroid_position) in
      &asteroids_query
    {
      let bullet_size = bullet_transform.scale.max_element();
      let asteroid_size = asteroid_transform.scale.max_element();
      let distance = (bullet_position.0 - asteroid_position.0).length();

      if distance < bullet_size / 2.0 + asteroid_size / 2.0 {
        commands.entity(bullet_entity).despawn();
        commands.entity(asteroid_entity).despawn();

        let asteroid_new_size = match asteroid.size {
          AsteroidSize::Big => Some(AsteroidSize::Medium),
          AsteroidSize::Medium => Some(AsteroidSize::Small),
          AsteroidSize::Small => None,
        };

        if let Some(asteroid_new_size) = asteroid_new_size {
          for _ in 0..2 {
            commands
              .spawn()
              .insert(Asteroid {
                size: asteroid_new_size,
              })
              .insert(Position(asteroid_position.0.clone()))
              .insert(Velocity(
                get_random_point().normalize() * ASTEROID_VELOCITY,
              ))
              .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                transform: Transform::default()
                  .with_translation(Vec3::new(0.0, 0.0, 2.0)),
                material: materials
                  .add(ColorMaterial::from(Color::rgba(0.8, 0.8, 0.8, 1.0))),
                ..default()
              });
          }
        }
      }
    }
  }
}

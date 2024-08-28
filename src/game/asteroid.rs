// esternal crates
use bevy::log::info;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

// internal crates
// while main.rs uses bevy_asteroid:: we use crate:: here because
// - we are referring to other modules in the SAME crate
// - whereas main.rs is OUTSIDE the bevy_asteroid crate
use crate::game::config::Config;
use crate::game::screen_wrap::ScreenWrap;

pub struct AsteroidPlugin {
    config: AsteroidConfig,
}

impl AsteroidPlugin {
    pub fn new(spawn_count: usize) -> Self {
        Self {
            config: AsteroidConfig::new(spawn_count),
        }
    }
}

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config)
            .add_plugins(ShapePlugin)
            .add_systems(
                Startup,
                (
                    spawn_asteroids,
                    // using Bevy system ordering to log after asteroids spawn
                    log_asteroid_spawn_count.after(spawn_asteroids),
                ),
            )
            .add_systems(FixedUpdate, (move_asteroids, rotate_asteroids));
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct AsteroidConfig {
    spawn_count: usize,
    // display
    num_sides: usize,
    fill_color: Color,
    stroke_color: Color,
    stroke_width: f32,
    // physics
    max_velocity: f32,
    max_angular_velocity: f32,
    // bound
    radius_min: f32,
    radius_max: f32,
}

impl AsteroidConfig {
    pub fn new(spawn_count: usize) -> Self {
        Self {
            spawn_count,
            ..Default::default()
        }
    }
}

impl Default for AsteroidConfig {
    fn default() -> Self {
        Self {
            spawn_count: 5,
            num_sides: 6,
            fill_color: Color::srgb(1.0, 0.0, 0.0),
            stroke_color: Color::srgb(0.0, 0.0, 1.0),
            stroke_width: 2.0,
            max_velocity: 100.0,
            max_angular_velocity: 2.0,
            radius_min: 25.0,
            radius_max: 150.0,
        }
    }
}

// Bevy components
#[derive(Component)] // attribute this as a Bevy component
struct Asteroid {
    velocity: Vec2,
    angular_velocity: f32,
}

fn spawn_asteroids(mut commands: Commands, config: Res<Config>, asteroid: Res<AsteroidConfig>) {
    let layer_id = config.layer_ids.enemy;
    (0..asteroid.spawn_count)
        .map(|i| i % layer_id as usize)
        .for_each(|id| {
            spawn_asteroid(&mut commands, &config, &asteroid, id as u8);
        });
}

/// Spawns a single asteroid with position and velocity (random)
fn spawn_asteroid(
    commands: &mut Commands,
    config: &Config,
    asteroid: &AsteroidConfig,
    layer_id: u8,
) {
    let mut rng = rand::thread_rng();
    let random_radius = rng.gen_range(asteroid.radius_min..asteroid.radius_max);
    let spawn_pos = Vec3::new(
        rng.gen_range(-config.bounds.x * 0.5..config.bounds.x * 0.5),
        rng.gen_range(-config.bounds.y * 0.5..config.bounds.y * 0.5),
        layer_id as f32,
    );
    let max_velocity = loop {
        let v = rng.gen_range(-asteroid.max_velocity..asteroid.max_velocity);
        // ensure that this isn't zero, else asteroid is spawned at a stand still
        if v.abs() >= 1.0 {
            break v;
        }
    };
    let max_angular_velocity =
        rng.gen_range(-asteroid.max_angular_velocity..asteroid.max_angular_velocity);

    // defines asteroid shape
    let shape = shapes::RegularPolygon {
        sides: asteroid.num_sides,
        feature: shapes::RegularPolygonFeature::Radius(random_radius),
        ..shapes::RegularPolygon::default()
    };

    // instantiate asteroid draw
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(
                    spawn_pos.x,
                    spawn_pos.y,
                    spawn_pos.z,
                )),
                ..default()
            },
            Fill::color(asteroid.fill_color),
            Stroke::new(asteroid.stroke_color, asteroid.stroke_width),
        ))
        // insert gameplay components
        .insert((
            Asteroid {
                // set random positional velocity
                velocity: Vec2::new(max_velocity, max_velocity),
                // set random rotational speed
                angular_velocity: max_angular_velocity,
            },
            ScreenWrap {
                border_radius: random_radius,
            },
        ));
}

/// System to rotate asteroids over time
fn rotate_asteroids(mut query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    for (mut transform, asteroid) in query.iter_mut() {
        transform.rotate_z(asteroid.angular_velocity * time.delta_seconds());
    }
}

fn move_asteroids(mut query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    for (mut transform, asteroid) in query.iter_mut() {
        transform.translation.x += asteroid.velocity.x * time.delta_seconds();
        transform.translation.y += asteroid.velocity.y * time.delta_seconds();
    }
}

fn log_asteroid_spawn_count(asteroid_config: Res<AsteroidConfig>) {
    info!(
        "[AsteroidPlugin] - SPAWN_ASTEROIDS - Count: {}",
        asteroid_config.spawn_count
    );
}

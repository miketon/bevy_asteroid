// extern crates
use bevy::log::info;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// internal crates
use crate::game::config::Config;

// Player component
#[derive(Component, Debug)]
struct Player {}

// IO Control Component
// @todo : log debug using reflection
#[derive(Component, Debug, Reflect)]
struct IoControl {
    rotation: f32,
    acceleration: f32,
}

impl Default for IoControl {
    fn default() -> Self {
        Self {
            rotation: 0.0,
            acceleration: 0.0,
        }
    }
}

// Ballistic Component
#[derive(Component, Debug)]
struct Ballistic {
    velocity: Vec2,
}

impl Default for Ballistic {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
        }
    }
}

// Player Draw for spawn and transform
// Reflect to :
// - create debug views
// - save/load functionality
// - tweak these values at runtime
#[derive(Resource, Clone, Debug, Reflect)]
struct Draw {
    radius: f32,
    rotation_speed: f32,
    thrust_speed: f32,
    max_speed: f32,
    friction: f32,
    fill_color: Color,
    stroke_color: Color,
    stroke_width: f32,
}

impl Default for Draw {
    fn default() -> Self {
        Self {
            radius: 25.0,
            rotation_speed: 5.0,
            thrust_speed: 300.0,
            max_speed: 500.0,
            friction: 0.98,
            fill_color: Color::srgb(0.0, 0.0, 1.0),
            stroke_color: Color::srgb(0.0, 1.0, 0.0),
            stroke_width: 2.0,
        }
    }
}

// Player Plugin
pub struct PlayerPlugin;

// defines player plugin and build method to:
// - insert resources
// - adding systems
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // insert resources before systems to ensure initial configuration
        app.insert_resource(Draw::default())
            .add_systems(
                Startup,
                (spawn_player, || info!("[PlayerPlugin] : STARTUP")),
            )
            .add_systems(Update, player_input)
            .add_systems(FixedUpdate, player_movement);
    }
}

fn spawn_player(mut commands: Commands, draw: Res<Draw>, config: Res<Config>) {
    info!("[PlayerPlugin] : SPAWN_PLAYER");
    // define player shape
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(draw.radius),
        ..shapes::RegularPolygon::default()
    };

    // instantiate player shape
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            // z = layer sorting
            spatial: SpatialBundle::from_transform(Transform::from_xyz(
                0.0,
                0.0,
                config.layer_ids.player as f32,
            )),
            ..default()
        },
        Fill::color(draw.fill_color),
        Stroke::new(draw.stroke_color, draw.stroke_width),
        Player {},
        IoControl::default(),
        Ballistic::default(),
    ));
}

fn player_input(mut query: Query<(&mut IoControl, &Player)>, input: Res<ButtonInput<KeyCode>>) {
    let Ok((mut io, _player)) = query.get_single_mut() else {
        warn!("Expected a single player, found none or multiple");
        return;
    };

    // Handle rotation
    let rotation = if input.pressed(KeyCode::ArrowLeft) {
        1.0
    } else if input.pressed(KeyCode::ArrowRight) {
        -1.0
    } else {
        0.0
    };

    // Handle acceleration
    let acceleration = if input.pressed(KeyCode::ArrowUp) {
        1.0
    } else {
        0.0
    };

    // Update player component
    io.rotation = rotation;
    io.acceleration = acceleration;
}

fn player_movement(
    mut query: Query<(&mut Transform, &mut Ballistic, &IoControl, &Player)>,
    draw: Res<Draw>,
    config: Res<Config>,
    time: Res<Time>,
) {
    // error check : early exit and warn if !single player found
    let Ok((mut transform, mut xform, io, _player)) = query.get_single_mut() else {
        warn!("Expected a single player, found none or multiple");
        return;
    };

    // Apply rotation
    // let rotation = transform.local_y().truncate() * io.rotation;
    transform.rotate_z(io.rotation * draw.rotation_speed * time.delta_seconds());

    // Update velocity
    // @note : we need a Vec2, and local_y() returns Vec3
    // - truncate() drops the vec.z and returns Vec2
    if io.acceleration > 0.0 {
        xform.velocity = transform.local_y().truncate() * draw.thrust_speed
    } else {
        xform.velocity *= draw.friction
    };

    // apply movement
    // @note : .translation is a Vec3 and .velocity is a Vec2
    // - .extend(0.0) adds .z value so we return a Vec3
    transform.translation +=
        xform.velocity.clamp_length_max(draw.max_speed).extend(0.0) * time.delta_seconds();

    wrap_position(&mut transform.translation, config.bounds * 0.5);
}

fn wrap_position(pos: &mut Vec3, bounds: Vec2) {
    if pos.x > bounds.x {
        pos.x = -bounds.x;
    } else if pos.x < -bounds.x {
        pos.x = bounds.x;
    }
    if pos.y > bounds.y {
        pos.y = -bounds.y;
    } else if pos.y < -bounds.y {
        pos.y = bounds.y;
    }
}

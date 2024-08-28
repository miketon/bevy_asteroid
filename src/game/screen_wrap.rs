use crate::game::config::Config;
use bevy::prelude::*;

/// Wrap around screen edge
/// -------------------------------------
///
///      -x_bound     0     +x_bound
///         |         |         |
///         v         v         v
/// --------+=========+=========+--------
///    <----| Game Area (visible) |---->
/// --------+=========+=========+--------
///         ^                   ^
///         |                   |
///   Wraps to here       Wraps to here
pub struct ScreenWrapPlugin;

impl Plugin for ScreenWrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, wrap_positions);
    }
}

#[derive(Component, Debug)]
pub struct ScreenWrap {
    // prevent large gameobjects from disappearing abruptly @border
    pub border_radius: f32,
}

impl Default for ScreenWrap {
    fn default() -> Self {
        Self { border_radius: 0.0 }
    }
}

fn wrap_positions(mut query: Query<(&mut Transform, &ScreenWrap)>, config: Res<Config>) {
    let bounds = config.bounds * 0.5;
    for (mut transform, wrap) in query.iter_mut() {
        let mut position = transform.translation;
        position.x = wrap_coordinate(position.x, bounds.x, wrap.border_radius);
        position.y = wrap_coordinate(position.y, bounds.y, wrap.border_radius);

        transform.translation = position;
    }
}

fn wrap_coordinate(coord: f32, screen_bound: f32, border_radius: f32) -> f32 {
    let bound = screen_bound + border_radius;
    if coord > bound {
        -bound + border_radius
    } else if coord < -bound {
        bound - border_radius
    } else {
        coord
    }
}

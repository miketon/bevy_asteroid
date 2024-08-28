use crate::game::config::Config;
use bevy::prelude::*;

pub struct ScreenWrapPlugin;

impl Plugin for ScreenWrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, wrap_positions);
    }
}

#[derive(Component)]
pub struct ScreenWrap;

fn wrap_positions(mut query: Query<&mut Transform, With<ScreenWrap>>, config: Res<Config>) {
    let bounds = config.bounds * 0.5;
    for mut transform in query.iter_mut() {
        let mut position = transform.translation;
        position.x = wrap_coordinate(position.x, bounds.x);
        position.y = wrap_coordinate(position.y, bounds.y);

        transform.translation = position;
    }
}

fn wrap_coordinate(coord: f32, bound: f32) -> f32 {
    if coord > bound {
        -bound
    } else if coord < -bound {
        bound
    } else {
        coord
    }
}

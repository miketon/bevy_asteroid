use bevy::log::info;
use bevy::prelude::*;

const LAYER_IDS: LayerIds = LayerIds {
    enemy: 253,
    player: 254,
};

/// Plugin for setting up Game Configuration
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::default()).add_systems(
            Startup,
            (configure_window, || info!("[ConfigPlugin] : LOADED")),
        );
    }
}

/// Configuration for game window:
/// - title
/// - boundaries
#[derive(Resource, Debug, Clone)]
pub struct Config {
    pub title: String,
    pub bounds: Vec2,
    pub layer_ids: LayerIds,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "ECS Asteroid".to_string(),
            bounds: Vec2::new(800.0, 600.0),
            layer_ids: LayerIds {
                enemy: LAYER_IDS.enemy,
                player: LAYER_IDS.enemy,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerIds {
    pub enemy: u8,
    pub player: u8,
}

/// Initializes display context
fn configure_window(mut windows: Query<&mut Window>, config: Res<Config>) {
    // @note : what is the diff with single_mut vs get_single_mut
    if let Ok(mut window) = windows.get_single_mut() {
        window.resolution.set(config.bounds.x, config.bounds.y);
        window.title = config.title.to_string();
    } else {
        error!("[ConfigPlugin] : Failed to get window");
    }
}

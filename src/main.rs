use bevy::log::info;
use bevy::prelude::*;
use bevy_asteroid::game::asteroid::AsteroidPlugin;
use bevy_asteroid::game::config::ConfigPlugin;
use bevy_asteroid::game::player::PlayerPlugin;

const ASTEROID_SPAWN_COUNT: usize = 3;

// Entry point for application
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // context for windows and game defaults
            ConfigPlugin,
            // handles Player
            PlayerPlugin,
            // handles Asteroids
            AsteroidPlugin::new(ASTEROID_SPAWN_COUNT),
        ))
        // @note : if we don't spawn camera, screen is blank
        .add_systems(Startup, (log_version, setup_camera))
        .run();
}

/// Set up the 2D camera
/// - Necessary for rendering game world
/// - Otherwise blank screen
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

struct VersionInfo {
    version: &'static str,
    git_hash: &'static str,
    git_message: &'static str,
    build_date: &'static str,
}

const VERSION_INFO: VersionInfo = VersionInfo {
    // CARGO_PKG_VERSION -> macro to get version from Cargo.toml
    version: env!("CARGO_PKG_VERSION"),
    git_hash: env!("GIT_HASH", "unknown"),
    git_message: env!("GIT_MESSAGE", "unknown"),
    build_date: env!("BUILD_DATE", "unknown"),
};

fn log_version() {
    info!(
        "[MAIN] Starting Bevy_Asteroid v{} (Built on: {}, Git: {})",
        VERSION_INFO.version, VERSION_INFO.build_date, VERSION_INFO.git_hash
    );
    info!("[MAIN] commit : {}", VERSION_INFO.git_message);
}

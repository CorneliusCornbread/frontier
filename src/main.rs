mod wasm;
mod world;

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        //.add_systems(FixedUpdate, wasm::system::update_wasi_scripts)
        .add_systems(
            Startup,
            (
                wasm::system::add_wasi_script,
                world::system::load_funny_gltf,
            ),
        );

    app.run();
}

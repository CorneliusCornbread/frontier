mod wasm;

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Update, wasm::system::update_wasi_scripts)
        .add_systems(Startup, wasm::system::add_wasi_script);

    app.run();
}

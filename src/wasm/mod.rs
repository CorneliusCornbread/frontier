use bevy::ecs::component::Component;
use wasmer::{imports, Instance, Module, Store};

pub mod api;
pub mod system;

#[derive(Component)]
pub struct WasmScript {
    pub module: Module,
    pub store: Store,
    pub instance: Instance,
}

impl WasmScript {
    pub fn new(data: &[u8]) -> WasmScript {
        let mut store = Store::default();
        let module = Module::new(&store, data).unwrap();

        let import_object = imports! {};
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();

        WasmScript {
            module,
            store,
            instance,
        }
    }
}

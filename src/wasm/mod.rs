use bevy::ecs::component::Component;
use wasmer::{Imports, Instance, Module, Store};

pub mod api;
pub mod system;

#[derive(Component)]
pub struct WasmScript {
    pub module: Module,
    pub store: Store,
    pub instance: Instance,
}

impl WasmScript {
    pub fn new(data: &[u8], mut store: Store, imports: &Imports) -> WasmScript {
        let module = Module::new(&store, data).unwrap();

        let instance = Instance::new(&mut store, &module, imports).unwrap();

        WasmScript {
            module,
            store,
            instance,
        }
    }
}

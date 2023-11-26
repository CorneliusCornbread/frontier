use bevy::{
    ecs::system::{Commands, Query},
    log,
};
use wasmer::{
    imports, wat2wasm, Function, FunctionEnv, FunctionEnvMut, FunctionType, Type, TypedFunction,
    Value,
};

use super::WasmScript;

// TODO: this is an example of how this might work, needs to be fleshed out.
pub fn update_wasi_scripts(mut wasm: Query<&mut WasmScript>) {
    for mut wasm_instance in wasm.iter_mut() {
        // The Wasm module exports a function called `sum`. Let's get it.
        let sum: TypedFunction<(i32, i32), i32> = wasm_instance
            .instance
            .exports
            .get_function("sum")
            .unwrap()
            .typed(&wasm_instance.store)
            .unwrap();

        log::error!("Calling `sum` function...");
        // Let's call the `sum` exported function. It will call each
        // of the imported functions.
        let result = sum.call(&mut wasm_instance.store, 10, 2).unwrap();
        log::error!("Result: {}", result);
    }
}

pub fn add_wasi_script(mut commands: Commands) {
    let wasm_bytes = wat2wasm(
        br#"
(module
  (func $multiply_dynamic (import "env" "multiply_dynamic") (param i32) (result i32))
  (func $multiply_typed (import "env" "multiply_typed") (param i32) (result i32))

  (type $sum_t (func (param i32) (param i32) (result i32)))
  (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
    (call $multiply_dynamic (local.get $x))
    (call $multiply_typed (local.get $y))
    i32.add)
  (export "sum" (func $sum_f)))
"#,
    )
    .unwrap();
    let mut store = wasmer::Store::default();

    struct MyEnv;
    let env = FunctionEnv::new(&mut store, MyEnv {});

    let multiply_dynamic_signature = FunctionType::new(vec![Type::I32], vec![Type::I32]);
    let multiply_dynamic = Function::new(&mut store, &multiply_dynamic_signature, |args| {
        log::error!("Calling `multiply_dynamic`...");

        let result = args[0].unwrap_i32() * 2;

        log::error!("Result of `multiply_dynamic`: {:?}", result);

        Ok(vec![Value::I32(result)])
    });

    fn multiply(_env: FunctionEnvMut<MyEnv>, a: i32) -> i32 {
        log::error!("Calling `multiply_typed`...");
        let result = a * 3;

        log::error!("Result of `multiply_typed`: {:?}", result);

        result
    }
    let multiply_typed = Function::new_typed_with_env(&mut store, &env, multiply);

    // Create an import object.
    let import_object = imports! {
        "env" => {
            "multiply_dynamic" => multiply_dynamic,
            "multiply_typed" => multiply_typed,
        }
    };

    let script = WasmScript::new(&wasm_bytes, store, &import_object);

    commands.spawn(script);
}

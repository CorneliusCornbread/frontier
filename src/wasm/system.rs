use bevy::ecs::system::{Commands, Query};
use wasmer::{imports, wat2wasm, Function, FunctionType, Type, Value};

use super::WasmScript;

pub fn update_wasi_scripts(mut commands: Commands, mut wasm: Query<&mut WasmScript>) {
    for mut wasm_instance in wasm.iter_mut() {
        let multiply_dynamic_signature = FunctionType::new(vec![Type::I32], vec![Type::I32]);
        let multiply_dynamic = Function::new(
            &mut wasm_instance.store,
            &multiply_dynamic_signature,
            |args| {
                println!("Calling `multiply_dynamic`...");

                let result = args[0].unwrap_i32() * 2;

                println!("Result of `multiply_dynamic`: {:?}", result);

                Ok(vec![Value::I32(result)])
            },
        );
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
    let script = WasmScript::new(&wasm_bytes);

    commands.spawn(script);
}

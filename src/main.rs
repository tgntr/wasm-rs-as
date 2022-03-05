mod exports;
use wasmer::{imports, Instance, Module, Store, Function};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let wasm: &[u8] = include_bytes!("../build/optimized.wasm");
    let store = Store::default();
    let module = Module::new(&store, &wasm)?;
    let import_object = imports! {
        "imports" => {
            "increment" => Function::new_native(&store, exports::increment),
        }
    };
    let instance = Instance::new(&module, &import_object)?;
    let add_func = instance.exports.get_native_function::<(i64, i64), i64>("add")?;
    let result = add_func.call(1, 1)?;
    Ok(assert_eq!(result, 3))
}

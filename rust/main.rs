use wasmer::{imports, Instance, Module, Store, Value, Function};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm: &[u8] = include_bytes!("../build/optimized.wasm");
    let store = Store::default();
    let module = Module::new(&store, &wasm)?;
    let import_object = imports! {
        "env" => {
            "increment" => Function::new_native(&store, increment),
        }
    };
    let instance = Instance::new(&module, &import_object)?;
    let wasm_add = instance.exports.get_function("add")?;
    let result = wasm_add.call(&[Value::I64(1), Value::I64(1)])?;
    assert_eq!(result[0], Value::I64(3));

    Ok(())
}

pub fn increment(number: u64) -> u64 {
    number + 1
}

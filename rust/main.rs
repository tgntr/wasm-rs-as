use wasmer::{imports, Instance, Module, Store, Function};

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
    let add_func = instance.exports.get_function("add")?.native::<(i64, i64), i64>()?;
    let result = add_func.call(1, 1)?;
    assert_eq!(result, 3);

    Ok(())
}

pub fn increment(number: u64) -> u64 {
    number + 1
}

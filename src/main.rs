use std::error::Error;
use wasmer::{imports, Array, Instance, Module, Store, WasmPtr};

fn main() -> Result<(), Box<dyn Error>> {
    let wasm = include_bytes!("../build/optimized.wasm");
    let store = Store::default();
    let module = Module::new(&store, &wasm)?;
    let instance = Instance::new(&module, &imports!{})?.exports;
    let memory = instance.get_memory("memory")?;
    let allocate_fn = instance.get_native_function::<u32, WasmPtr<u8, Array>>("allocate")?;
    let read_fn = instance.get_native_function::<u32, WasmPtr<u8, Array>>("read")?;
    let data = b"memory example";
    let data_size = data.len() as u32;

    let allocate = allocate_fn.call(data_size)?;
    match allocate.deref(memory, allocate.offset(), data_size) {
        Some(cells) => cells.iter().enumerate().for_each(|(i, c)| c.set(data[i])),
        None => Err("allocate failed")?,
    }

    let read = read_fn.call(allocate.offset())?;
    let result = match read.deref(memory, read.offset(), data_size) {
        Some(cells) => cells.iter().map(|c| c.get()).collect::<Vec<u8>>(),
        None => Err("read failed")?,
    };
    assert_eq!(result, data);
    Ok(())
}


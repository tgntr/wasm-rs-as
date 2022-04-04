use std::error::Error;
use wasmer::{imports, Array, Instance, Module, Store, WasmPtr};
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    // instantiate wasm module in wasmer runtime
    let wasm = include_bytes!("../build/contract.wasm");
    let store = Store::default();
    let module = Module::new(&store, &wasm)?;
    let instance = Instance::new(&module, &imports! {})?.exports;
    let memory = instance.get_memory("memory")?;

    // allocate new region in module memory
    let allocate_fn = instance.get_native_function::<u32, WasmPtr<types::Region>>("allocate")?;
    let region_wptr = allocate_fn.call(5)?.deref(memory).unwrap();
    let mut region = region_wptr.get();

    // get region offset
    let offset_wptr = WasmPtr::<u8, Array>::new(region.offset);
    let offset = offset_wptr
        .deref(memory, offset_wptr.offset(), region.capacity)
        .unwrap();

    // save new msg to region
    let msg = &b"test".to_vec();
    let msg_len = msg.len() as u32;
    msg.iter().enumerate().for_each(|(i, m)| offset[i].set(*m));

    // update region length
    region.length = msg_len;
    region_wptr.set(region);

    // query region offset
    let result_wptr = WasmPtr::<u8, Array>::new(region.offset);
    let result = result_wptr
        .deref(memory, result_wptr.offset(), msg_len)
        .unwrap()
        .iter()
        .map(|c| c.get())
        .collect::<Vec<u8>>();

    // assert that region state is updated
    assert_eq!(msg, &result);
    assert_eq!(msg_len, region_wptr.get().length);
    Ok(())
}

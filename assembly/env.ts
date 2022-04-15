// fn debug(&self, message: &str) {
//     // keep the boxes in scope, so we free it at the end (don't cast to pointers same line as build_region)
//     let region = build_region(message.as_bytes());
//     let region_ptr = region.as_ref() as *const Region as u32;
//     unsafe { debug(region_ptr) };
// 

export declare function debug(ptr: usize): void;
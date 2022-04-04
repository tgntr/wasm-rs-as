use wasmer::ValueType;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Region {
    pub offset: u32,
    pub capacity: u32,
    pub length: u32,
}

unsafe impl ValueType for Region {}
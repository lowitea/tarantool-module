pub const MP_DATETIME: std::os::raw::c_char = 4;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct datetime {
    pub s: i64,
    pub n: i32,
    pub tz: i16,
    pub tzi: i16,
}

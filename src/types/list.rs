use std::ffi::c_void;

use super::key::UplayKey;
use super::save::UplaySave;

#[repr(C)]
pub struct UplayList {
    pub count: u32,
    pub list: List,
}

#[repr(C)]
pub union List {
    pub items: *const *const c_void,
    pub keys: *const *const UplayKey,
    pub saves: *const *const UplaySave,
}

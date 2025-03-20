use serde::{Deserialize, Serialize};

use crate::core::Hachimi;

pub fn is_il2cpp_lib(filename: &str) -> bool {
    filename.ends_with("libil2cpp.so")
}

pub fn is_criware_lib(filename: &str) -> bool {
    filename.ends_with("libcri_ware_unity.so")
}

pub fn on_hooking_finished(_hachimi: &Hachimi) {
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "Config::default_menu_open_key")]
    pub menu_open_key: i32
}

impl Config {
    fn default_menu_open_key() -> i32 { 22 /* KEYCODE_DPAD_RIGHT */ }
}
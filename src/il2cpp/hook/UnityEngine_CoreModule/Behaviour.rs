use crate::il2cpp::{api::il2cpp_resolve_icall, types::*};

static mut SET_ENABLED_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_enabled, SET_ENABLED_ADDR, (), this: *mut Il2CppObject, value: bool);

pub fn init(_UnityEngine_CoreModule: *const Il2CppImage) {
    unsafe {
        SET_ENABLED_ADDR = il2cpp_resolve_icall(c"UnityEngine.Behaviour::set_enabled(System.Boolean)".as_ptr());
    }
}
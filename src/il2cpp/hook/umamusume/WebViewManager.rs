use std::ptr::null_mut;

use crate::{core::ext::StringExt, il2cpp::{symbols::{get_method_addr, MonoSingleton}, types::*}};

use super::{DialogCommon, TextId};

static mut CLASS: *mut Il2CppClass = null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

pub fn instance() -> *mut Il2CppObject {
    let Some(singleton) = MonoSingleton::new(class()) else {
        return null_mut();
    };
    singleton.instance()
}

static mut OPEN_ADDR: usize = 0;
impl_addr_wrapper_fn!(Open, OPEN_ADDR, (),
    this: *mut Il2CppObject, url: *mut Il2CppString, dialog_data: *mut Il2CppObject,
    on_loaded_callback: *mut Il2CppDelegate, on_error_callback: *mut Il2CppDelegate,
    is_direct: bool
);

pub fn quick_open(dialog_title: &str, url: &str) {
    let dialog_data = DialogCommon::Data::new();
    DialogCommon::Data::SetSimpleOneButtonMessage(
        dialog_data,
        dialog_title.to_il2cpp_string(),
        null_mut(),
        null_mut(),
        TextId::from_name("Common0007"),
        9 // BIG_ONE_BUTTON
    );

    let web_view_manager = instance();
    Open(web_view_manager,
        url.to_il2cpp_string(),
        dialog_data, null_mut(), null_mut(), false
    )
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, WebViewManager);

    unsafe {
        CLASS = WebViewManager;
        OPEN_ADDR = get_method_addr(WebViewManager, cstr!("Open"), 5);
    }
}
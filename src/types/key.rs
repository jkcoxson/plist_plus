// jkcoxson

use std::{ffi::CString, os::raw::c_char};

use log::trace;

use crate::{unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Gets the key plist value
    /// Current uses of this are unknown
    pub fn get_key_val(&self) -> Result<String, ()> {
        if self.plist_type != PlistType::Key {
            return Err(());
        }
        let mut key = std::ptr::null_mut();
        trace!("Getting key value");
        unsafe { unsafe_bindings::plist_get_key_val(self.plist_t, &mut key) };
        trace!("Converting key to string");
        let key = unsafe { std::ffi::CStr::from_ptr(key).to_string_lossy().into_owned() };
        Ok(key)
    }

    /// Sets the key plist value
    /// Current uses of this are unknown
    pub fn set_key_val(&self, key: &str) {
        let key = CString::new(key).unwrap();
        trace!("Setting key value");
        unsafe { unsafe_bindings::plist_set_key_val(self.plist_t, key.as_ptr() as *const c_char) }
    }
}

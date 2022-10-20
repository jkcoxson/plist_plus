// jkcoxson

use std::{ffi::CString, os::raw::c_char};

use log::trace;

use crate::{error::PlistError, unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Creates a new plist with type string
    pub fn new_string(string: &str) -> Plist {
        trace!("Generating new string plist");
        let string = match CString::new(string) {
            Ok(s) => s,
            Err(_) => {
                panic!("Could not convert string to CString");
            }
        };
        unsafe { unsafe_bindings::plist_new_string(string.as_ptr() as *const c_char) }.into()
    }
    /// Returns the value of the string
    pub fn get_string_val(&self) -> Result<String, PlistError> {
        if self.plist_type != PlistType::String {
            return Err(PlistError::InvalidArg);
        }
        let mut val = std::ptr::null_mut();
        trace!("Getting string value");
        unsafe { unsafe_bindings::plist_get_string_val(self.plist_t, &mut val) };
        trace!("Converting cstring to string");
        let val = unsafe {
            std::ffi::CString::from_raw(val)
                .to_str()
                .unwrap()
                .to_string()
        };
        Ok(val)
    }
    /// Returns a C pointer to a CString containing the value of the string
    /// # Safety
    /// Don't be stupid
    pub unsafe fn get_string_ptr(&self) -> *const c_char {
        unsafe_bindings::plist_get_string_ptr(self.plist_t, std::ptr::null_mut())
    }
    /// Sets a plist to type string with the given value
    pub fn set_string_val(&self, val: &str) {
        let val = CString::new(val).unwrap();
        trace!("Setting string value");
        unsafe {
            unsafe_bindings::plist_set_string_val(self.plist_t, val.as_ptr() as *const c_char)
        }
    }
}

impl From<String> for Plist {
    fn from(plist_data: String) -> Self {
        Plist::new_string(&plist_data)
    }
}

impl From<&String> for Plist {
    fn from(plist_data: &String) -> Self {
        Plist::new_string(plist_data)
    }
}

impl From<&str> for Plist {
    fn from(plist_data: &str) -> Self {
        Plist::new_string(plist_data)
    }
}

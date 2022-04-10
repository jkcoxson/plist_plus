// jkcoxson

use std::ffi::CString;

use crate::{debug, unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Creates a new plist with type string
    pub fn new_string(string: &str) -> Plist {
        debug!("Generating new string plist");
        let string = match CString::new(string) {
            Ok(s) => s,
            Err(_) => {
                panic!("Could not convert string to CString");
            }
        };
        unsafe { unsafe_bindings::plist_new_string(string.as_ptr() as *const i8) }.into()
    }
    /// Returns the value of the string
    pub fn get_string_val(&self) -> Result<String, ()> {
        if self.plist_type != PlistType::String {
            return Err(());
        }
        let mut val = std::ptr::null_mut();
        debug!("Getting string value");
        unsafe { unsafe_bindings::plist_get_string_val(self.plist_t, &mut val) };
        debug!("Converting cstring to string");
        let val = unsafe { std::ffi::CStr::from_ptr(val).to_string_lossy().into_owned() };
        Ok(val)
    }
    /// Returns a C pointer to a CString containing the value of the string
    pub unsafe fn get_string_ptr(&self) -> *const i8 {
        unsafe_bindings::plist_get_string_ptr(self.plist_t, std::ptr::null_mut())
    }
    /// Sets a plist to type string with the given value
    pub fn set_string_val(&self, val: &str) {
        let val = CString::new(val).unwrap();
        debug!("Setting string value");
        unsafe { unsafe_bindings::plist_set_string_val(self.plist_t, val.as_ptr() as *const i8) }
    }
}

impl From<String> for Plist {
    fn from(plist_data: String) -> Self {
        let s = Plist::new_string(&plist_data);
        s
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

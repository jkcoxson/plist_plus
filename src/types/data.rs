// jkcoxson

use std::os::raw::c_char;

use log::{info, warn};

use crate::{unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Returns a new plist with a type of data
    /// The data type is equivalent to a collection of bytes
    pub fn new_data(data: &[u8]) -> Plist {
        info!("Generating new data plist");
        unsafe {
            unsafe_bindings::plist_new_data(
                data.as_ptr() as *const c_char,
                std::convert::TryInto::try_into(data.len()).unwrap(),
            )
        }
        .into()
    }
    /// Returns the data value contained in a plist
    pub fn get_data_val(&self) -> Result<Vec<c_char>, ()> {
        if self.plist_type != PlistType::Data {
            return Err(());
        }
        let mut val = std::ptr::null_mut();
        let mut size = 0;
        info!("Getting data value");
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, &mut val, &mut size);
        }
        let val = unsafe { std::slice::from_raw_parts(val, size as usize) };
        Ok(val.to_vec())
    }
    /// Sets the contents of a plist to the given data
    pub fn set_data_val(&self, val: &[c_char]) -> Result<(), ()> {
        if self.plist_type != PlistType::Data {
            warn!("Cannot set value of non-data plist");
            return Err(());
        }
        info!("Setting data value");
        unsafe { unsafe_bindings::plist_set_data_val(self.plist_t, val.as_ptr(), val.len() as u64) }
        Ok(())
    }
}

impl From<Vec<u8>> for Plist {
    fn from(plist_data: Vec<u8>) -> Self {
        Plist::new_data(&plist_data)
    }
}

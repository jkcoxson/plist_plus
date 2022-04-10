// jkcoxson

use crate::{debug, Plist, PlistType, unsafe_bindings};

impl Plist {
    pub fn new_data(data: &[u8]) -> Plist {
        debug!("Generating new data plist");
        unsafe {
            unsafe_bindings::plist_new_data(
                data.as_ptr() as *const i8,
                std::convert::TryInto::try_into(data.len()).unwrap(),
            )
        }
        .into()
    }
    pub fn get_data_val(&self) -> Result<Vec<i8>, ()> {
        if self.plist_type != PlistType::Data {
            return Err(());
        }
        let mut val = std::ptr::null_mut();
        let mut size = 0;
        debug!("Getting data value");
        unsafe {
            unsafe_bindings::plist_get_data_val(self.plist_t, &mut val, &mut size);
        }
        let val = unsafe { std::slice::from_raw_parts(val, size as usize) };
        Ok(val.to_vec())
    }
    pub fn set_data_val(&self, val: &[i8]) -> Result<(), ()> {
        if self.plist_type != PlistType::Data {
            debug!("Cannot set value of non-data plist");
            return Err(());
        }
        debug!("Setting data value");
        unsafe { unsafe_bindings::plist_set_data_val(self.plist_t, val.as_ptr(), val.len() as u64) }
        Ok(())
    }
}

impl From<Vec<u8>> for Plist {
    fn from(plist_data: Vec<u8>) -> Self {
        let len = plist_data.len();
        let plist_data = plist_data.as_ptr() as *const i8;
        let plist_t = unsafe { std::mem::zeroed() };
        debug!("Creating plist from binary data");
        unsafe { unsafe_bindings::plist_from_bin(plist_data, len as u32, plist_t) };
        unsafe { (*plist_t).into() }
    }
}

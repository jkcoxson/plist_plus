// jkcoxson

use crate::{debug, Plist, PlistType, unsafe_bindings};

impl Plist {
    pub fn new_uid(uid: u64) -> Plist {
        debug!("Generating new plist uid");
        unsafe { unsafe_bindings::plist_new_uid(uid) }.into()
    }
    pub fn get_uid_val(&self) -> Result<u64, ()> {
        if self.plist_type != PlistType::Uid {
            return Err(());
        }
        let mut val = unsafe { std::mem::zeroed() };
        debug!("Getting uid value");
        unsafe {
            unsafe_bindings::plist_get_uid_val(self.plist_t, &mut val);
        }
        Ok(val)
    }
    pub fn set_uid_val(&self, val: u64) {
        debug!("Setting uid value");
        unsafe { unsafe_bindings::plist_set_uid_val(self.plist_t, val) }
    }
}
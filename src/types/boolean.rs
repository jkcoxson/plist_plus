// jkcoxson

use log::info;

use crate::unsafe_bindings;
use crate::Plist;
use crate::PlistType;

impl Plist {
    /// Returns a plist with bool type
    pub fn new_bool(bool: bool) -> Plist {
        info!("Generating new bool plist");
        unsafe {
            unsafe_bindings::plist_new_bool(match bool == true {
                true => 1,
                false => 0,
            })
        }
        .into()
    }
    /// Returns the value of the bool
    pub fn get_bool_val(&self) -> Result<bool, ()> {
        if self.plist_type != PlistType::Boolean {
            return Err(());
        }
        let val = unsafe { std::mem::zeroed() };
        info!("Getting bool value");
        Ok(unsafe {
            unsafe_bindings::plist_get_bool_val(self.plist_t, val);
            match *val {
                0 => false,
                _ => true,
            }
        })
    }
    /// Sets a plist to type bool with the given value
    pub fn set_bool_val(&self, val: bool) {
        let val = if val { 1 } else { 0 };
        info!("Setting bool value");
        unsafe { unsafe_bindings::plist_set_bool_val(self.plist_t, val) }
    }
}

impl TryFrom<Plist> for bool {
    type Error = ();
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_bool_val()
    }
}

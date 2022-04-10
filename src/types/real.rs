// jkcoxson

use crate::{debug, unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Creates a new plist with type float
    pub fn new_real(real: f64) -> Plist {
        debug!("Generating new float plist");
        unsafe { unsafe_bindings::plist_new_real(real) }.into()
    }
    /// Returns the value of the float
    pub fn get_real_val(&self) -> Result<f64, ()> {
        if self.plist_type != PlistType::Real {
            return Err(());
        }
        let val = unsafe { std::mem::zeroed() };
        debug!("Getting float value");
        Ok(unsafe {
            unsafe_bindings::plist_get_real_val(self.plist_t, val);
            *val
        })
    }
    /// Sets a plist to type float with the given value
    pub fn set_real_val(&self, val: f64) -> Result<(), ()> {
        debug!("Setting float value");
        unsafe { unsafe_bindings::plist_set_real_val(self.plist_t, val) }
        Ok(())
    }
}

impl TryFrom<Plist> for f64 {
    type Error = ();
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_real_val()
    }
}

impl From<f64> for Plist {
    fn from(val: f64) -> Self {
        Plist::new_real(val)
    }
}

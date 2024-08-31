// jkcoxson

use log::trace;

use crate::{error::PlistError, unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Creates a new plist with type float
    pub fn new_real(real: f64) -> Plist {
        trace!("Generating new float plist");
        unsafe { unsafe_bindings::plist_new_real(real) }.into()
    }
    /// Returns the value of the float
    pub fn get_real_val(&self) -> Result<f64, PlistError> {
        if self.plist_type != PlistType::Real {
            return Err(PlistError::InvalidArg);
        }
        let mut val = unsafe { std::mem::zeroed() };
        trace!("Getting float value");
        Ok(unsafe {
            unsafe_bindings::plist_get_real_val(self.plist_t, &mut val);
            val
        })
    }
    /// Sets a plist to type float with the given value
    pub fn set_real_val(&self, val: f64) -> Result<(), PlistError> {
        trace!("Setting float value");
        unsafe { unsafe_bindings::plist_set_real_val(self.plist_t, val) }
        Ok(())
    }
}

impl TryFrom<Plist> for f64 {
    type Error = PlistError;
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_real_val()
    }
}

impl From<f64> for Plist {
    fn from(val: f64) -> Self {
        Plist::new_real(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_test() {
        let p = Plist::new_real(3.1415926);
        p.set_real_val(1234.098765).unwrap();
        assert_eq!(p.get_real_val().unwrap(), 1234.098765)
    }
}

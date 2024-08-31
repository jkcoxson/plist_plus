// jkcoxson

use log::trace;

use crate::{error::PlistError, unsafe_bindings, Plist, PlistType};

impl Plist {
    /// Creates a new plist with the type of an integer
    pub fn new_uint(uint: u64) -> Plist {
        trace!("Generating new uint plist");
        unsafe { unsafe_bindings::plist_new_uint(uint) }.into()
    }
    /// Sets the plist as type integer with the given value
    pub fn set_uint_val(&self, val: u64) {
        trace!("Setting uint value");
        unsafe { unsafe_bindings::plist_set_uint_val(self.plist_t, val) }
    }
    /// Returns the value of the integer
    pub fn get_uint_val(&self) -> Result<u64, PlistError> {
        if self.plist_type != PlistType::Integer {
            return Err(PlistError::InvalidArg);
        }
        let mut val = unsafe { std::mem::zeroed() };
        trace!("Getting uint value");
        Ok(unsafe {
            unsafe_bindings::plist_get_uint_val(self.plist_t, &mut val);
            val
        })
    }
}

impl TryFrom<Plist> for u64 {
    type Error = PlistError;
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_uint_val()
    }
}

impl From<u64> for Plist {
    fn from(val: u64) -> Self {
        Plist::new_uint(val)
    }
}

impl TryFrom<Plist> for u32 {
    type Error = PlistError;
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_uint_val().map(|val| val as u32)
    }
}

impl From<u32> for Plist {
    fn from(val: u32) -> Self {
        Plist::new_uint(val as u64)
    }
}

impl TryFrom<Plist> for u16 {
    type Error = PlistError;
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_uint_val().map(|val| val as u16)
    }
}

impl From<u16> for Plist {
    fn from(val: u16) -> Self {
        Plist::new_uint(val as u64)
    }
}

impl TryFrom<Plist> for u8 {
    type Error = PlistError;
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_uint_val().map(|val| val as u8)
    }
}

impl From<u8> for Plist {
    fn from(val: u8) -> Self {
        Plist::new_uint(val as u64)
    }
}

impl TryFrom<Plist> for usize {
    type Error = PlistError;
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_uint_val().map(|x| x as usize)
    }
}

impl From<usize> for Plist {
    fn from(val: usize) -> Self {
        Plist::new_uint(val as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_test() {
        let p = Plist::new_uint(123412340987);
        p.set_uint_val(098709781234);
        assert_eq!(p.get_uint_val().unwrap(), 098709781234);
    }
}

// jkcoxson

use crate::{debug, Plist, PlistType, unsafe_bindings};


impl Plist {
    pub fn new_uint(uint: u64) -> Plist {
        debug!("Generating new uint plist");
        unsafe { unsafe_bindings::plist_new_uint(uint) }.into()
    }
    pub fn set_uint_val(&self, val: u64) {
        debug!("Setting uint value");
        unsafe { unsafe_bindings::plist_set_uint_val(self.plist_t, val) }
    }
    pub fn get_uint_val(&self) -> Result<u64, ()> {
        if self.plist_type != PlistType::Integer {
            return Err(());
        }
        let val = unsafe { std::mem::zeroed() };
        debug!("Getting uint value");
        Ok(unsafe {
            unsafe_bindings::plist_get_uint_val(self.plist_t, val);
            *val
        })
    }
}

impl TryFrom<Plist> for u64 {
    type Error = ();
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
    type Error = ();
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
    type Error = ();
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
    type Error = ();
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
    type Error = ();
    fn try_from(plist: Plist) -> Result<Self, Self::Error> {
        plist.get_uint_val().map(|x| x as usize)
    }
}

impl From<usize> for Plist {
    fn from(val: usize) -> Self {
        Plist::new_uint(val as u64)
    }
}
